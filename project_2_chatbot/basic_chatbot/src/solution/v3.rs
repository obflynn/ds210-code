use kalosm::language::*;
use std::collections::HashMap;

#[allow(dead_code)]
pub struct ChatbotV3 {
    model: Llama, 
    sessions: HashMap<String, Chat<Llama>>, 
}
impl ChatbotV3 {
    #[allow(dead_code)]
    pub fn new(model: Llama) -> ChatbotV3 {
        return ChatbotV3 {
            model: model, sessions: HashMap::new() // initialize HashMap to store chat session data for each user
        };
    }


    #[allow(dead_code)]
    pub async fn chat_with_user(&mut self, username: String, message: String) -> String {
        if let Some(chat_sesh) = self.sessions.get_mut(&username) { // check if the user has an existing chat session (sesh) and retrieve username/session data if it exists
            let output = chat_sesh.add_message(message).await; // add user's message to chat sesh data

            match output { // respond to user with model output unless there's an error
                Ok(response) => return response, 
                Err(_) => return String::from("Error! Model couldn't generate response"),
            }
        } 
        else { // if the user has not messaged the bot before, create a new chat sesh for that user and add their messages to the session data
            let mut chat_sesh = self.model.chat();
            let output = chat_sesh.add_message(message).await;
            
            match output {
                Ok(response) => {
                    self.sessions.insert(username, chat_sesh); // store chat messages in session HashMap -> key = username, value = message data
                    return response;
                },
                Err(_) => return String::from("Error! Model couldn't generate response"),
            }
        }
    }

    pub fn get_history(&self, username: String) -> Vec<String> {
     if let Some(chat) = self.sessions.get(&username) {

        if let Ok(session) = chat.session() {

            let history = session.history();

            let mut messages = Vec::new();

            for msg in history {
                messages.push(format!("{:?}", msg));
            }

            return messages;
        }
    }
        return Vec::new();
    }
}
