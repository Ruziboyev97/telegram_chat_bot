// src/bot.rs

use async_openai::Client;
use async_openai::config::OpenAIConfig;
use teloxide::prelude::*;
use teloxide::dptree::deps;

use crate::{config, openai_handler};

/// Обработчик входящих сообщений.
async fn message_handler(
    bot: Bot,
    msg: Message,
    client: Client<OpenAIConfig>, // Pass the client directly without 'In'
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let user_question = msg.text().unwrap_or_default();

    if user_question.is_empty() {
        bot.send_message(msg.chat.id, "Пожалуйста, задайте мне вопрос.").await?;
        return Ok(());
    }

    // Показываем статус "печатает..."
    bot.send_chat_action(msg.chat.id, teloxide::types::ChatAction::Typing).await?;

    // Задаем вопрос OpenAI
    match openai_handler::ask_question(&client, user_question).await {
        Ok(answer) => {
            bot.send_message(msg.chat.id, answer).await?;
        }
        Err(e) => {
            log::error!("OpenAI API error: {:?}", e);
            bot.send_message(
                msg.chat.id,
                "Извините, произошла ошибка при обращении к AI.",
            ).await?;
        }
    }

    Ok(())
}

/// Основная функция запуска бота.
pub async fn run() {
    let config = match config::load_config_from_db().await {
        Ok(cfg) => {
            log::info!("Configuration loaded successfully.");
            cfg
        }
        Err(e) => {
            log::error!("Failed to load configuration: {}", e);
            return;
        }
    };

    let bot = Bot::new(config.telegram_token);

    // Create the OpenAI client with the new configuration method
    let openai_config = OpenAIConfig::new().with_api_key(config.openai_api_key);
    let openai_client = Client::with_config(openai_config);

    // Create a dispatcher with the handler and dependencies
    let handler = dptree::entry()
        .branch(Update::filter_message().endpoint(message_handler));

    // Start the dispatcher with your client as a singleton dependency
    Dispatcher::builder(bot, handler)
        .dependencies(deps![openai_client]) // Use the new deps macro
        .enable_ctrlc_handler()
        .build()
        .dispatch()
        .await;
}