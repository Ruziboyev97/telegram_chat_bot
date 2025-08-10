// src/openai_handler.rs

use async_openai::{
    Client,
    types::{CreateChatCompletionRequestArgs, Role},
};

/// Отправляет вопрос в OpenAI и возвращает ответ в виде строки.
pub async fn ask_question(
    client: &Client,
    question: &str,
) -> Result<String, Box<dyn std::error::Error + Send + Sync>> {
    let request = CreateChatCompletionRequestArgs::default()
        .model("gpt-4o")
        .messages([
            async_openai::types::ChatCompletionRequestMessageArgs::default()
                .role(Role::User)
                .content(question)
                .build()?,
        ])
        .build()?;

    let response = client.chat().create(request).await?;

    if let Some(choice) = response.choices.first() {
        if let Some(content) = &choice.message.content {
            return Ok(content.clone());
        }
    }

    Ok("Sorry, I couldn't get a proper response.".to_string())
}
