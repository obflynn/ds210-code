use kalosm::language::*;

#[allow(dead_code)]
pub struct ChatbotV2 {
     chat_session: Chat<Llama>,
}

impl ChatbotV2 {
    #[allow(dead_code)]
    pub fn new(model: Llama) -> ChatbotV2 {
        let chat_session = model
            .chat();
        

        return ChatbotV2 {
            chat_session
        };
    }

    #[allow(dead_code)]
    pub async fn chat_with_user(&mut self, message: String) -> String {
          // send the message to the bot
    let output = self.chat_session.add_message(message).await;

    //get the response if there's no error
    if let Ok(response) = output {
        return response;
    }
        return String::from("Hello, I am not a bot (yet)!");
    }
}