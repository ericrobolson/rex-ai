mod agent;
mod chatllm;
mod repl;

use clap::Parser;
use serde::{Deserialize, Serialize};
use std::env;
use std::error::Error;
use std::time::Duration;

const OPENAI_KEY: &'static str = "OPENAI_API_KEY";

/// A CLI for interacting with OpenAI's GPT-3 API.
#[derive(Parser, Clone, Debug, PartialEq)]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// Signals whether to parse out code or not.
    #[arg(short, long)]
    code: bool,

    /// The prompt to use.
    #[arg(last = true)]
    prompt: Vec<String>,
}

pub fn api_key() -> String {
    env::var(OPENAI_KEY).expect(&format!("Expected '{OPENAI_KEY} to be set!"))
}

fn main() -> Result<(), Box<dyn Error>> {
    let api_key = env::var(OPENAI_KEY).expect(&format!("Expected '{OPENAI_KEY} to be set!"));

    let args = Cli::parse();

    if args.prompt.is_empty() {
        println!("Starting interactive mode...");
        repl::run()?;
        return Ok(());
    }

    let body = Body {
        model: "gpt-3.5-turbo".to_string(),
        messages: vec![Message::user(args.prompt.join(" "))],
        temperature: 0.7,
    };
    let body = serde_json::to_string(&body)?;

    let client = reqwest::blocking::Client::new();
    let response = client
        .post("https://api.openai.com/v1/chat/completions")
        .header("Content-Type", "application/json")
        .header("Authorization", format!("Bearer {}", api_key))
        .body(body)
        .timeout(Duration::from_secs(30))
        .send()?;

    if response.status() != reqwest::StatusCode::OK {
        println!("Error: {:?}", response.text());
        return Ok(());
    }

    let response: serde_json::Value = serde_json::from_str(&response.text().unwrap())?;

    let response = response["choices"][0]["message"]["content"]
        .as_str()
        .unwrap_or_default();

    let response = if args.code {
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

    println!("{}", response);

    Ok(())
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
