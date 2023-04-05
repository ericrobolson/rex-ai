use serde::{Deserialize, Serialize};
use std::error::Error;
use std::time::Duration;

#[derive(Debug, Clone)]
pub struct ChatLLM {}
impl ChatLLM {
    pub fn new() -> Self {
        Self {}
    }

    pub fn generate(
        &mut self,
        input: &str,
        stop_pattern: Option<Vec<String>>,
    ) -> Result<String, Box<dyn Error>> {
        let parse_code = false;
        let body = Body {
            model: "gpt-3.5-turbo".to_string(),
            messages: vec![Message::user(input.to_string())],
            temperature: 0.7,
        };
        let body = serde_json::to_string(&body)?;

        let client = reqwest::blocking::Client::new();
        let response = client
            .post("https://api.openai.com/v1/chat/completions")
            .header("Content-Type", "application/json")
            .header("Authorization", format!("Bearer {}", crate::api_key()))
            .body(body)
            .timeout(Duration::from_secs(30))
            .send()?;

        if response.status() != reqwest::StatusCode::OK {
            panic!("Error: {:?}", response.text());
        }

        let response: serde_json::Value = serde_json::from_str(&response.text().unwrap())?;

        let response = response["choices"][0]["message"]["content"]
            .as_str()
            .unwrap_or_default();

        let response = if parse_code {
            let split = response.split("```").collect::<Vec<&str>>();
            if split.is_empty() {
                response.to_string()
            } else if split.len() == 1 {
                split[0].to_string()
            } else {
                split[1].to_string()
            }
        } else {
            response.to_string()
        };

        Ok(response)
    }
}

/// A message sent to ChatGPT.
#[derive(Debug, Serialize, Deserialize)]
struct Message {
    role: String,
    content: String,
}
impl Message {
    pub fn user(content: String) -> Self {
        Self {
            role: "user".to_string(),
            content,
        }
    }
}

/// The body of a request to ChatGPT.
#[derive(Debug, Serialize, Deserialize)]
struct Body {
    model: String,
    messages: Vec<Message>,
    temperature: f32,
}
