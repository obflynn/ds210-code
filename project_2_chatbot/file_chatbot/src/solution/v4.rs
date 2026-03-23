use kalosm::language::*;
use crate::solution::file_library::{self, save_chat_session_to_file};

pub struct ChatbotV4 {
    model: Llama,
}

impl ChatbotV4 {
    pub fn new(model: Llama) -> ChatbotV4 {
        return ChatbotV4 {
            model: model,
        };
    }

    pub async fn chat_with_user(&mut self, username: String, message: String) -> String {
        let filename = &format!("{}.txt", username);

        let mut chat_session: Chat<Llama> = self.model
            .chat()
            .with_system_prompt("The assistant will act like a pirate");
        
        let output = chat_session.add_message(message).await; // model generates response to user message
        
        match output { // match statement 1: model responds to user's message unless there's an error
            Ok(response) => { // match statement 2: if the model is able to generate a response, check if the user has an existing chat session file so the current chat session can be added
                match file_library::load_chat_session_from_file(&filename) { // check if user has an existing chat session file
                    Some(session) => { // if the user has a chat session file, save the current chat session to that file
                        file_library::save_chat_session_to_file(filename, &session); 
                    }
                    None => { // report that no chat session file exists for the user
                        return String::from("No chat history found for user!");
                    }
                }
               return response; // return the model generated response to user
            },
            Err(_) => return String::from("Hello, I am not a bot (yet)!"),
        }

    }

    pub fn get_history(&self, username: String) -> Vec<String> {
        let filename = &format!("{}.txt", username);

        match file_library::load_chat_session_from_file(&filename) {
            None => {
                return Vec::new();
            },
            Some(session) => {
                // TODO: what should happen here?
                return Vec::new();
            }
        }
    }
}
