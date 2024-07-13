use crate::completer::Completer;
use reqwest::blocking::Client;
use serde::Deserialize;
use std::env;

pub struct GptCompleter;

#[derive(Deserialize)]
struct OpenAIResponse {
    choices: Vec<Choice>,
}

#[derive(Deserialize)]
struct Choice {
    text: String,
}

impl GptCompleter {
    fn call_openai_api(prompt: &str) -> Result<String, Box<dyn std::error::Error>> {
        let api_key = env::var("OPENAI_API_KEY")?;
        let client = Client::new();

        let response = client
            .post("https://api.openai.com/v1/completions")
            .header("Content-Type", "application/json")
            .header("Authorization", format!("Bearer {}", api_key))
            .json(&serde_json::json!({
                "model": "gpt-3.5-turbo-instruct",
                "prompt": prompt,
                "temperature": 0,
                "max_tokens": 9,
                "top_p": 1,
                "frequency_penalty": 0,
                "presence_penalty": 0
            }))
            .send()?
            .json::<OpenAIResponse>()?;

        if let Some(choice) = response.choices.get(0) {
            return Ok(prompt.to_owned() + &choice.text.clone());
        }
        Err("Failed".into())
    }
}

impl Completer for GptCompleter {
    fn complete(&self, input: &str) -> String {
        match GptCompleter::call_openai_api(input) {
            Ok(response) => response.trim().to_string(),
            Err(_) => "CONNECTION FAILED".to_string(),
        }
    }
}
