// src/openai_handler.rs

use async_openai::{
    Client,
    types::{
        ChatCompletionRequestMessage, // Add this import for the general message type
        CreateChatCompletionRequestArgs,
        Role,
        ChatCompletionRequestUserMessageArgs,
    },
    config::OpenAIConfig,
};

/// Отправляет вопрос в OpenAI и возвращает ответ в виде строки.
pub async fn ask_question(
    client: &Client<OpenAIConfig>,
    question: &str,
) -> Result<String, Box<dyn std::error::Error + Send + Sync>> {
    let user_message = ChatCompletionRequestUserMessageArgs::default()
        .role(Role::User)
        .content(question)
        .build()?;

    // Explicitly convert the User message into the general ChatCompletionRequestMessage enum
    let messages: Vec<ChatCompletionRequestMessage> = vec![user_message.into()];

    let request = CreateChatCompletionRequestArgs::default()
        .model("gpt-4o")
        .messages(messages) // Pass the Vec<ChatCompletionRequestMessage> here
        .build()?;

    let response = client.chat().create(request).await?;

    if let Some(choice) = response.choices.first() {
        if let Some(content) = &choice.message.content {
            return Ok(content.clone());
        }
    }

    Ok("Sorry, I couldn't get a proper response.".to_string())
}