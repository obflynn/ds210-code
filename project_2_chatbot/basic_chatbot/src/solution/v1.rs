use kalosm::language::*; // kalosm is an async library

#[allow(dead_code)]
pub struct ChatbotV1 {
    model: Llama,
}

impl ChatbotV1 {
    #[allow(dead_code)]
    pub fn new(model: Llama) -> ChatbotV1 {
        return ChatbotV1 { model: model };
    }

    #[allow(dead_code)] // prevents rust from showing irrelevant warnings for the chat_with_userfn
    // async fns allow other functions to run while waiting for this model's response => saves time and improves efficiency
    pub async fn chat_with_user(&mut self, message: String) -> String { 
        let mut chat_session: Chat<Llama> = self.model
            .chat()
            .with_system_prompt("The assistant will act like a pirate");

    // I'm unsure if the following code works as intended due to technical difficulties w/ VS code          

        if message.is_empty() { // if no message is sent (aka initial state), then return default response
            return String::from("Hello, I am not a bot (yet)!");
        }
        else {
            let async_output = chat_session.add_message(message); // adds user's message to the chat and creates async response
            let next_output = async_output.await.unwrap(); // waits for model to generate resoponse, then checks if response is valid type
            return String::from(next_output.as_str()); // converts the response to a string and returns it
        }
    }
}

