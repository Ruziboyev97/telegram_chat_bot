// src/config.rs

use sqlx::postgres::PgPoolOptions;
use sqlx::Error as SqlxError;
use std::collections::HashMap;

/// Структура для хранения наших токенов
#[derive(Debug, Clone)]
pub struct Config {
    pub telegram_token: String,
    pub gemini_api_key: String, // Переименовано для ясности
}

/// Асинхронная функция для загрузки конфигурации из PostgreSQL
pub async fn load_config_from_db() -> Result<Config, SqlxError> {
    let db_url = "postgresql://neondb_owner:npg_yTKMQJ13eEnl@ep-old-brook-abe7xrx6.eu-west-2.aws.neon.tech/neondb?sslmode=require";
    log::info!("Connecting to the database...");

    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(db_url)
        .await?;

    let rows = sqlx::query!("SELECT key, value FROM config")
        .fetch_all(&pool)
        .await?;

    let mut config_map: HashMap<String, String> = HashMap::new();
    for row in rows {
        config_map.insert(row.key, row.value);
    }

    let telegram_token = config_map
        .get("TELEGRAM_TOKEN")
        .ok_or_else(|| {
            log::error!("TELEGRAM_TOKEN not found in database");
            SqlxError::RowNotFound
        })?
        .clone();

    let gemini_api_key = config_map
        .get("GEMINI_API_KEY")
        .ok_or_else(|| {
            log::error!("GEMINI_API_KEY not found in database");
            SqlxError::RowNotFound
        })?
        .clone();

    log::info!("Configuration loaded: telegram_token and gemini_api_key found");

    let config = Config {
        telegram_token,
        gemini_api_key,
    };

    // Закрываем соединение с БД
    pool.close().await;

    Ok(config)
}
