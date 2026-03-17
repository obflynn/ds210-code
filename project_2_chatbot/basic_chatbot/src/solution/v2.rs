use kalosm::language::*;

#[allow(dead_code)]
pub struct ChatbotV2 {
     chat_session: Chat<Llama>, // field to store chat session history w/in model
}

impl ChatbotV2 {
    #[allow(dead_code)]
    pub fn new(model: Llama) -> ChatbotV2 {
        let chat_session = model
            .chat(); // creates a new chat session from the model
        

        return ChatbotV2 {
            chat_session // initialize chatbot struct with chat session data from the model
        };
    }

    #[allow(dead_code)]
    pub async fn chat_with_user(&mut self, message: String) -> String {
    let output = self.chat_session.add_message(message).await; // store chat session data w/in the chatbot struct 

    //get the response if there's no error
    if let Ok(response) = output {
        return response; 
    }
        return String::from("Hello, I am not a bot (yet)!");
    }
}