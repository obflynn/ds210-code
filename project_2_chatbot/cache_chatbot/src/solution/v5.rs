use kalosm::language::*;
use file_chatbot::solution::file_library;

use crate::solution::Cache;

pub struct ChatbotV5 {
    model: Llama,
    cache: Cache<Chat<Llama>>,
}

impl ChatbotV5 {
    pub fn new(model: Llama) -> ChatbotV5 {
        return ChatbotV5 {
            model: model,
            cache: Cache::new(3),
        };
    }

    pub async fn chat_with_user(&mut self, username: String, message: String) -> String {
        let filename = &format!("{}.txt", username);
        let cached_chat = self.cache.get_chat(&username);

        match cached_chat {
            None => {
                println!("chat_with_user: {username} is not in the cache!");
                // The cache does not have the chat. What should you do?
                return String::from("Hello, I am not a bot (yet)!");
            }
            Some(chat_session) => {
                println!("chat_with_user: {username} is in the cache! Nice!");
                // The cache has this chat. What should you do?
                return String::from("Hello, I am not a bot (yet)!");

            }
        }
    }

    pub fn get_history(&mut self, username: String) -> Vec<String> {
        let filename = &format!("{}.txt", username);
        let cached_chat = self.cache.get_chat(&username);

        match cached_chat {
            None => { 
                println!("get_history: {username} is not in the cache!");
                self.cache.insert_chat(username, self.model.chat()); // if the user doesn't have a chat session in the cache, then create one and insert it into the cache
                return Vec::new(); // if the user doesn't have a chat session in the cache, then they don't have any chat history, so return an empty vector
            }
            Some(chat_session) => {
                println!("get_history: {username} is in the cache! Nice!");
                let history = chat_session.session().unwrap().history(); // 
                
                let mut messages: Vec<String> = Vec::new(); // vector to store each chat message in user's history 
                for msg_i in history { // iterate through chats in history 
                    messages.push(msg_i.content().to_string()); // push the content of each chat into the messages vector
                }
                return messages; // final returned item is a vector of strings (aka user's chat history)
            }
        }
    }
}