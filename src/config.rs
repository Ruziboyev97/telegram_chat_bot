// src/config.rs

use tokio_postgres::{Error as PostgresError, NoTls};

/// Структура для хранения наших токенов
#[derive(Debug, Clone)]
pub struct Config {
    pub telegram_token: String,
    pub openai_api_key: String,
}

/// Асинхронная функция для загрузки конфигурации из PostgreSQL
pub async fn load_config_from_db() -> Result<Config, PostgresError> {
    // Прямая ссылка на базу данных
    let db_url = "postgresql://neondb_owner:npg_yTKMQJ13eEnl@ep-old-brook-abe7xrx6-pooler.eu-west-2.aws.neon.tech/neondb?sslmode=require&channel_binding=require";
    log::info!("Connecting to the database...");

    // Подключаемся к базе данных
    let (client, connection) = tokio_postgres::connect(db_url, NoTls).await?;

    // Запускаем соединение в фоновой задаче
    tokio::spawn(async move {
        if let Err(e) = connection.await {
            log::error!("Database connection error: {}", e);
        }
    });

    // Выполняем SQL-запрос
    let row = client
        .query_one(
            "SELECT TELEGRAM_TOKEN, OPENAI_API_KEY FROM config LIMIT 1",
            &[],
        )
        .await?;

    // Создаем и возвращаем нашу структуру Config
    let config = Config {
        telegram_token: row.get("TELEGRAM_TOKEN"),
        openai_api_key: row.get("OPENAI_API_KEY"),
    };

    Ok(config)
}
