// src/bot.rs
use teloxide::dptree::deps;
use teloxide::prelude::*;

use crate::{config, gemini_handler, gemini_handler::GeminiClient};

/// Обработчик входящих сообщений.
async fn message_handler(
    bot: Bot,
    msg: Message,
    client: GeminiClient,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    // Проверяем, что сообщение является текстом.
    let user_question = match msg.text() {
        Some(text) => text,
        None => {
            return Ok(());
        }
    };

    if user_question.is_empty() {
        bot.send_message(msg.chat.id, "Пожалуйста, задайте мне вопрос.")
            .await?;
        return Ok(());
    }

    // Показываем статус "печатает..."
    bot.send_chat_action(msg.chat.id, teloxide::types::ChatAction::Typing)
        .await?;

    // Задаем вопрос Gemini
    match gemini_handler::ask_question(&client, user_question).await {
        Ok(answer) => {
            // Telegram имеет лимит на длину сообщения (4096 символов)
            if answer.len() > 4000 {
                let chunks: Vec<&str> = answer
                    .as_bytes()
                    .chunks(4000)
                    .map(|chunk| std::str::from_utf8(chunk).unwrap_or(""))
                    .collect();

                for chunk in chunks {
                    if !chunk.is_empty() {
                        bot.send_message(msg.chat.id, chunk).await?;
                        tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;
                    }
                }
            } else {
                bot.send_message(msg.chat.id, answer).await?;
            }
        }
        Err(e) => {
            log::error!("Gemini API error: {:?}", e);
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

    // Создаем клиента Gemini
    let gemini_client = GeminiClient::new(config.gemini_api_key);

    // Создаем диспетчер с обработчиком и зависимостями
    let handler = dptree::entry().branch(Update::filter_message().endpoint(message_handler));

    // Запускаем диспетчер с нашим клиентом как зависимостью
    Dispatcher::builder(bot, handler)
        .dependencies(deps![gemini_client])
        .enable_ctrlc_handler()
        .build()
        .dispatch()
        .await;
}
