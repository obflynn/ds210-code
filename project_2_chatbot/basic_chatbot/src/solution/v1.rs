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

#[allow(dead_code)]
pub async fn chat_with_user(&mut self, message: String) -> String {
    //creates a new chat session from the model
    let mut chat_session: Chat<Llama> = self.model
        .chat()
        // prompt
        .with_system_prompt("The assistant will act like a pirate");
    // send the user's message to the chat session
    let asynchronous_output = chat_session.add_message(message);
    let output = asynchronous_output.await;

    // use match to handle the result returned from the model
    match output {
        Ok(response) => response,
        Err(_) => String::from("Error: could not get response from model"),
    }
}
}
