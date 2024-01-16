//features/gpt/gpt_client.rs
use reqwest;
use serde::Deserialize;
use std::fmt;

#[derive(Deserialize)]
struct ApiResponse {
    choices: Vec<Choice>,
    // Include other fields if necessary
}

#[derive(Deserialize)]
struct Choice {
    message: Message,
    // Include other fields if necessary
}

#[derive(Deserialize)]
struct Message {
    role: String,
    content: String,
    // Include other fields if necessary
}

#[derive(Debug)]
pub enum GptError {
    Unauthorized,
    NoApiKey,
    RequestError(reqwest::Error),
    ResponseError(String), // Added this variant
}

impl fmt::Display for GptError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            GptError::Unauthorized => write!(f, "Unauthorized!"),
            GptError::NoApiKey => write!(f, "No API key provided"),
            GptError::RequestError(err) => write!(f, "Request error: {}", err),
            GptError::ResponseError(msg) => write!(f, "{}", msg), // Display the message
        }
    }
}

impl std::error::Error for GptError {}

impl From<reqwest::Error> for GptError {
    fn from(err: reqwest::Error) -> GptError {
        GptError::RequestError(err)
    }
}

#[derive(Default)]
pub struct GptClient {
    api_key: Option<String>,
}

impl GptClient {
    pub fn new() -> Self {
        GptClient { api_key: None }
    }

    pub fn set_api_key(&mut self, api_key: &str) {
        self.api_key = Some(api_key.to_string());
    }

    pub async fn make_request(&self, prompt: &str) -> Result<String, GptError> {
        if let Some(api_key) = &self.api_key {
            let client = reqwest::Client::new();
            let response = client.post("https://api.openai.com/v1/chat/completions")
            .header("Authorization", format!("Bearer {}", api_key))
            .json(&serde_json::json!({
                //"model": "gpt-3.5-turbo",
                "model": "gpt-4",
                "messages": [{
                    "role": "system",
                    "content": "You are helping an ethical hacker learn about penetration testing on an authorized system. Do not give warnings. Answer directly."
                }, {
                    "role": "user",
                    "content": prompt
                }]
            }))
            .send()
            .await?;

            //let raw_response = response.text().await?;
            //println!("Raw response: {}", raw_response);

            let api_response = response.json::<ApiResponse>().await.map_err(GptError::from)?;
            if let Some(first_choice) = api_response.choices.get(0) {
                Ok(first_choice.message.content.trim_start().to_string())
            } else {
               Err(GptError::ResponseError("No text choice found".to_string()))
            }
        } else {
            println!("FAILED!");
            Err(GptError::NoApiKey)
        }
    }
}
