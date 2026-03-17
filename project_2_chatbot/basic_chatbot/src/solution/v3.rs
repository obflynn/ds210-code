use kalosm::language::*;
use std::collections::HashMap;

#[allow(dead_code)]
pub struct ChatbotV3 {
     model: Llama,
    sessions: HashMap<String, Chat<Llama>>,
}

    #[allow(dead_code)]
    pub async fn chat_with_user(&mut self, username: String, message: String) -> String {
         // creates a new session for a new user
        if !self.sessions.contains_key(&username) {
            let chat_session = self
                .model
                .chat();
            

            self.sessions.insert(username.clone(), chat_session);
        }

        // if it's an existing user, retrieve the session for this user
        let session = self.sessions.get_mut(&username).unwrap();

        // Send the message
        let output = session.add_message(message).await;

        if let Ok(response) = output {
            return response;
        }

        return String::from("Hello, I am not a bot (yet)!");
    }

    #[allow(dead_code)]
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
