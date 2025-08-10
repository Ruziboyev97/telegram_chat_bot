// src/bot.rs

use async_openai::Client;
use teloxide::prelude::*;

use crate::{config, openai_handler};

/// Обработчик входящих сообщений.
async fn message_handler(
    bot: Bot,
    msg: Message,
    client: Client,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let user_question = msg.text().unwrap_or_default();

    if user_question.is_empty() {
        bot.send_message(msg.chat.id, "Пожалуйста, задайте мне вопрос.")
            .await?;
        return Ok(());
    }

    // Показываем статус "печатает..."
    bot.send_chat_action(msg.chat.id, teloxide::types::ChatAction::Typing)
        .await?;

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
            )
            .await?;
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
    let openai_client = Client::new().with_api_key(config.openai_api_key);

    // `dispatching::repl` — это элегантный способ запустить бота
    teloxide::repl(bot, move |bot: Bot, msg: Message| {
        // Клонируем клиент OpenAI для каждого нового сообщения
        let client = openai_client.clone();
        async move { message_handler(bot, msg, client).await }
    })
    .await;
}
