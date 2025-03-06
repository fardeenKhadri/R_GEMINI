use reqwest::Client;
use serde_json::{json, Value};
use std::env;
use dotenv::dotenv;
use std::io::{self, Write}; // For user input handling
use anyhow::{Result, anyhow}; // Better error handling

async fn send_message(text: &str) -> Result<String> {
    dotenv().ok(); // Load environment variables

    let api_key = env::var("GEMINI_API_KEY").expect("❌ API key not found in environment variables. Make sure .env is set.");
    let url = format!(
        "https://generativelanguage.googleapis.com/v1beta/models/gemini-1.5-flash:generateContent?key={}",
        api_key
    );

    let client = Client::new();
    let body = json!({
        "contents": [
            {
                "parts": [{ "text": text }]
            }
        ]
    });

    let response = client.post(&url).json(&body).send().await?;

    if response.status().is_success() {
        let json_response: Value = response.json().await?;

        // Extract AI response from JSON
        if let Some(reply) = json_response["candidates"][0]["content"]["parts"][0]["text"].as_str() {
            return Ok(reply.to_string());
        }
        
        return Ok(" (No valid response received)".to_string());
    } else {
        let status = response.status();
        let error_message = response.text().await.unwrap_or_else(|_| "Unknown error".to_string());

        eprintln!("❌ API Error: HTTP {} - {}", status, error_message);
        
        return Err(anyhow!("API Error: HTTP {} - {}", status, error_message)); // Use anyhow to return errors properly
    }
}

#[tokio::main(flavor = "multi_thread")]
async fn main() {
    dotenv().ok(); // Ensure dotenv is loaded

    println!(" SPOOKY Chatbot Initialized! Type your message (OkaY na).");

    loop {
        print!("👤 SPOOKY: ");
        io::stdout().flush().unwrap(); // Flush output before user input

        let mut user_input = String::new();
        io::stdin().read_line(&mut user_input).unwrap();
        let user_input = user_input.trim();

        if user_input.eq_ignore_ascii_case("exit") {
            println!("OkaY prends! BYE.");
            break;
        }

        println!("⏳ Thinking...");

        match send_message(user_input).await {
            Ok(response) => println!(" AI: {}\n", response),
            Err(e) => eprintln!("❌ Error: {:?}", e),
        }
    }
}
