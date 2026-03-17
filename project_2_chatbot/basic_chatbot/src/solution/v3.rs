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
            model: model, 
            sessions: HashMap::new(), // initialize HashMap to store chat session data for each user
        };
    }

    #[allow(dead_code)]
    pub async fn chat_with_user(&mut self, username: String, message: String) -> String {
        if let Some(chat_sesh) = self.sessions.get_mut(&username) {
            // existing chat session
            let output = chat_sesh.add_message(message).await; 

            match output { 
                Ok(response) => return response, 
                Err(_) => return String::from("Error! Model couldn't generate response"),
            }
        } else { 
            // new chat session for this user
            let mut chat_sesh = self.model.chat();
            let output = chat_sesh.add_message(message).await;
            
            match output {
                Ok(response) => {
                    self.sessions.insert(username, chat_sesh); 
                    return response;
                },
                Err(_) => return String::from("Error! Model couldn't generate response"),
            }
        }
    }

    #[allow(dead_code)]
pub fn get_history(&self, username: String) -> Vec<String> {
    if let Some(chat) = self.sessions.get(&username) {

        // session() returns Result --> must match on it
        if let Ok(session) = chat.session() {

            // history() returns Vec<ChatMessage>
            let history = session.history();

            let mut messages: Vec<String> = Vec::new();

            // iterate through ChatMessage
            for msg in history {
                messages.push(msg.content().to_string());
            }

            println!("{:?}", messages); 
            return messages;
        }
    }

    Vec::new()
}
} 