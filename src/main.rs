// src/main.rs

// Объявляем наши модули, чтобы Rust знал о них
mod bot;
mod config;
mod openai_handler;

#[tokio::main]
async fn main() {
    // Инициализируем логгер
    pretty_env_logger::init();

    log::info!("🚀 Bot is starting...");

    // Запускаем основную логику бота из модуля `bot`
    bot::run().await;
}
