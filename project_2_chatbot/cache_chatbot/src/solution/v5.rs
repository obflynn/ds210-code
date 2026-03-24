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

        let cached = self.cache.get_chat(&username); // check cache first for username

        let mut chat_session = match cached { // if the username is found in the cache, then that session data is taken into account during model's response generation
            Some(session) => {
                println!("Found in cache for user {username}");
                session.clone()
            }
            None => {
                println!("Not in cache for user {username}, loading from file or starting new session");
                match load_chat_session_from_file(&username) { // if username is not found in the cache, then check the file system
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

        let cached = self.cache.get_chat(&username); // check cache first for username

        let chat_session = match cached { 
            Some(session) => { // if username is found in cache, then that session data is used to return history
                println!("Found in cache for user {username}");
                session.clone() // clone the chat session data for the user from the cache
            }
            None => {
                println!("Not in cache for user {username}, loading from file or starting new session");
                match load_chat_session_from_file(&username) { // if username is not found in the cache, then check the file system
                    Some(session) => Chat::new(self.model.clone()).with_session(session), // if a chat session file is found, then return that data
                    None => Chat::new(self.model.clone()) // if no chat session is found for a given username, then return a new chat session
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

