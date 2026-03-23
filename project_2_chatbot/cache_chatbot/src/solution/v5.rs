use kalosm::language::*;
use file_chatbot::solution::file_library::{load_chat_session_from_file, save_chat_session_to_file};
use crate::solution::Cache;

pub struct ChatbotV5 {
    model: Llama,
    cache: Cache<Chat<Llama>>,
}

impl ChatbotV5 {
    pub fn new(model: Llama) -> ChatbotV5 {
        ChatbotV5 {
            model,
            cache: Cache::new(3),
        }
    }

    pub async fn chat_with_user(&mut self, username: String, message: String) -> String {
        println!("chat_with_user called for {username}");

        let cached = self.cache.get_chat(&username);

        let mut chat_session = match cached {
            Some(session) => {
                println!("Found in cache for user {username}");
                session.clone()
            }
            None => {
                 println!("Not in cache for user {username}, loading from file or starting new session");
                match load_chat_session_from_file(&username) {
                    Some(session) => Chat::new(self.model.clone()).with_session(session),
                    None => Chat::new(self.model.clone())
                        .with_system_prompt("The assistant will act like a pirate"),
                }
            }
        };

        // generate response
        let output = chat_session.add_message(message).await;
        let response = match output {
            Ok(text) => text,
            Err(_) => "Hmm, I have no answer.".to_string(),
        };

        // update cache
        self.cache.insert_chat(username.clone(), chat_session.clone());

        // save to file
        let session = chat_session.session().unwrap().clone();
        save_chat_session_to_file(&username, &session);

        response
    }

    pub fn get_history(&mut self, username: String) -> Vec<String> {
        println!("get_history called for {username}");

        let cached = self.cache.get_chat(&username);

        let chat_session = match cached {
            Some(session) => {
                println!("Found in cache for user {username}");
                session.clone()
            }
            None => {
                println!("Not in cache for user {username}, loading from file or starting new session");
                match load_chat_session_from_file(&username) {
                    Some(session) => Chat::new(self.model.clone()).with_session(session),
                    None => Chat::new(self.model.clone())
                        .with_system_prompt("The assistant will act like a pirate"),
                }
            }
        };

        // update cache
        self.cache.insert_chat(username.clone(), chat_session.clone());

        // return history (skip system prompt)
        chat_session
            .session()
            .unwrap()
            .history()
            .iter()
            .skip(1)
            .map(|msg| msg.content().to_string())
            .collect()
    }
}

