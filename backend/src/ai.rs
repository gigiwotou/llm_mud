use serde::{Serialize, Deserialize};
use reqwest::Client;

#[derive(Serialize, Deserialize)]
pub struct ChatRequest {
    pub messages: Vec<Message>,
    pub model: String,
    pub temperature: f32,
}

#[derive(Serialize, Deserialize)]
pub struct Message {
    pub role: String,
    pub content: String,
}

#[derive(Serialize, Deserialize)]
pub struct ChatResponse {
    pub choices: Vec<Choice>,
}

#[derive(Serialize, Deserialize)]
pub struct Choice {
    pub message: Message,
}

pub struct AIService {
    client: Client,
    api_url: String,
    api_key: String,
    model: String,
}

impl AIService {
    pub fn new(api_url: &str, api_key: &str, model: &str) -> Self {
        Self {
            client: Client::new(),
            api_url: api_url.to_string(),
            api_key: api_key.to_string(),
            model: model.to_string(),
        }
    }

    pub async fn generate_response(&self, messages: Vec<Message>) -> Result<String, reqwest::Error> {
        let request = ChatRequest {
            messages,
            model: self.model.clone(),
            temperature: 0.7,
        };

        let response = self.client
            .post(&self.api_url)
            .header("Content-Type", "application/json")
            .header("Authorization", format!("Bearer {}", self.api_key))
            .json(&request)
            .send()
            .await?
            .json::<ChatResponse>()
            .await?;

        Ok(response.choices[0].message.content.clone())
    }

    pub async fn get_npc_response(&self, npc_name: &str, npc_prompts: &[String], player_message: &str) -> Result<String, reqwest::Error> {
        let mut messages = vec![
            Message {
                role: "system".to_string(),
                content: format!("你是{}，{}", npc_name, npc_prompts.join(", ")),
            },
            Message {
                role: "user".to_string(),
                content: player_message.to_string(),
            },
        ];

        self.generate_response(messages).await
    }
}
