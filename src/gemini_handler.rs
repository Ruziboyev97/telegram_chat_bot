// src/gemini_handler.rs
use anyhow::Result;
use reqwest::Client;
use serde::{Deserialize, Serialize};

#[derive(Clone)]
pub struct GeminiClient {
    client: Client,
    api_key: String,
}

impl GeminiClient {
    pub fn new(api_key: String) -> Self {
        Self {
            client: Client::new(),
            api_key,
        }
    }
}

#[derive(Serialize)]
struct GeminiRequest {
    contents: Vec<Content>,
}

#[derive(Serialize)]
struct Content {
    parts: Vec<Part>,
}

#[derive(Serialize)]
struct Part {
    text: String,
}

#[derive(Deserialize)]
struct GeminiResponse {
    candidates: Option<Vec<Candidate>>,
    error: Option<GeminiError>,
}

#[derive(Deserialize)]
struct Candidate {
    content: ResponseContent,
}

#[derive(Deserialize)]
struct ResponseContent {
    parts: Vec<ResponsePart>,
}

#[derive(Deserialize)]
struct ResponsePart {
    text: String,
}

#[derive(Deserialize)]
struct GeminiError {
    message: String,
}

/// Отправляет вопрос в Gemini API и возвращает ответ в виде строки.
pub async fn ask_question(client: &GeminiClient, question: &str) -> Result<String> {
    // Используем актуальную модель gemini-1.5-flash
    let url = format!(
        "https://generativelanguage.googleapis.com/v1beta/models/gemini-1.5-flash:generateContent?key={}",
        client.api_key
    );

    let request_body = GeminiRequest {
        contents: vec![Content {
            parts: vec![Part {
                text: question.to_string(),
            }],
        }],
    };

    let response = client
        .client
        .post(&url)
        .json(&request_body)
        .send()
        .await?;

    if !response.status().is_success() {
        let error_text = response.text().await?;
        return Err(anyhow::anyhow!("Gemini API error: {}", error_text));
    }

    let gemini_response: GeminiResponse = response.json().await?;

    if let Some(error) = gemini_response.error {
        return Err(anyhow::anyhow!("Gemini API error: {}", error.message));
    }

    if let Some(candidates) = gemini_response.candidates {
        if let Some(candidate) = candidates.first() {
            if let Some(part) = candidate.content.parts.first() {
                return Ok(part.text.clone());
            }
        }
    }

    Err(anyhow::anyhow!("No response from Gemini API"))
}