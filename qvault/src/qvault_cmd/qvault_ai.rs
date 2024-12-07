use reqwest::blocking::Client;
use serde_json::json;
use std::env;
use dotenv::dotenv;
use std::fs;
use serde_json::Value;

use crate::qvault_log;
use crate::qvault_log::log_info;

pub fn chat_with_openai(message: &str) -> Result<String, Box<dyn std::error::Error>> {
    let mut api_key = env::var("QVAULT_OPENAI_API_KEY").ok();
    if api_key.is_none() {
        // Check if qvault.env exists and load it
        if fs::metadata("qvault.env").is_ok() {
            dotenv::from_path("qvault.env").ok();  // Load the .env file
            api_key = env::var("QVAULT_OPENAI_API_KEY").ok();  // Retry fetching the API key
        }
    }
    let client = Client::new();
    let url = "https://api.openai.com/v1/chat/completions";

    let payload = json!({
        "model": "gpt-3.5-turbo",
        "messages": [
            { "role": "user", "content": message }
        ]
    });

    let response = client
    .post(url)
    .header("Content-Type", "application/json")
    .header("Authorization", format!("Bearer {}", api_key.unwrap()))
    .json(&payload)
    .send()?
    .text()?;

    // Parse the response JSON
    let json_response: Value = serde_json::from_str(&response)?;
    if let Some(content) = json_response["choices"]
        .get(0)
        .and_then(|choice| choice["message"]["content"].as_str())
    {
        Ok(content.to_string())
    } else {
        Err("Failed to extract response content from OpenAI API".into())
    }
}
