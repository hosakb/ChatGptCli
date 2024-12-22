use futures_util::StreamExt;

use anyhow::{anyhow, Context, Result};
use reqwest::Client;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Debug)]
struct ApiResponse {
    id: String,
    object: String,
    created: i32,
    model: String,
    system_fingerprint: String,
    choices: Vec<Choice>,
}
#[derive(Deserialize, Debug)]
struct Choice {
    index: u32,
    delta: Delta,
    logprobs: Option<String>,
    finish_reason: Option<String>,
}
#[derive(Deserialize, Debug)]
struct Delta {
    content: Option<String>,
}

#[derive(Serialize)]
// #[serde(rename_all = "kebab-case")]
pub enum Model {
    Gpt4oMini,
    #[serde(rename = "gpt-4o")]
    Gpt4o,
    GptO1,
}

#[derive(Serialize)]
#[serde(rename_all = "lowercase")]
enum Role {
    USER,
    ASSISTANT,
}

#[derive(Serialize)]
struct Message {
    role: Role,
    content: String,
}

impl Message {
    pub fn new(role: Role, content: String) -> Message {
        Message { role, content }
    }
}

#[derive(Serialize)]
pub struct Body {
    model: Model,
    messages: Vec<Message>,
    stream: bool,
}

impl Body {
    pub fn new(model: Model, init_message: String) -> Body {
        let init_message = Message::new(Role::USER, init_message);
        let messages = vec![init_message];

        Body {
            model,
            messages,
            stream: true,
        }
    }
}

pub struct GptClient {
    client: Client,
    base_url: String,
    api_key: String,
}

impl GptClient {
    pub fn new(base_url: String, api_key: String) -> GptClient {
        GptClient {
            client: Client::new(),
            base_url,
            api_key,
        }
    }

    pub async fn send_prompt(&self, body: Body) -> Result<String> {
        let json = serde_json::to_string(&body)?;

        let response = self
            .client
            .post(self.base_url.clone())
            .header("Content-Type", "application/json")
            .header("Authorization", format!("Bearer {}", self.api_key))
            .body(json)
            .send()
            .await
            .context("during request.")?;

        if !response.status().is_success() {
            return Err(anyhow!("Bad Request {}", response.status()));
        }

        let mut stream = response.bytes_stream();

        while let Some(chunk_res) = stream.next().await {
            match chunk_res {
                Ok(chunk) => {

                    if let Ok(raw_response) = String::from_utf8(chunk.to_vec()) {

                        let filtered_response: Vec<ApiResponse> = raw_response
                            .lines()
                            .filter(|line| line.starts_with("data: ") && !line.contains("[DONE]"))
                            .map(|line| line.trim_start_matches("data: ").to_string())
                            .map(|line| {
                                serde_json::from_str::<ApiResponse>(line.as_str())
                                    .context("Failed to deserialize json response.")
                            })
                            .collect::<Result<Vec<ApiResponse>>>()?;

                            filtered_response.into_iter().for_each(|line| {
                                line.choices.into_iter().for_each(|choice| {
                                    if let Some(content) = choice.delta.content {
                                        print!("{}", content.replace("\"", ""));
                                    }
                                });
                            });
                    } else {
                        return Err(anyhow!("Failed to read chunk: {:?}", chunk));
                    }
                }
                Err(e) => return Err(anyhow!("Failed to get chunk: {:?}", e)),
            }
        }

        Ok(String::default())
    }
}
