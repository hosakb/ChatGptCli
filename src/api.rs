use anyhow::{anyhow, Context, Result};
use reqwest::Client;
use tokio_stream::{Stream, StreamExt};

use crate::model::{ApiResponse, Body};

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

    pub fn fetch_response_as_stream<'a>(
        &self,
        body: Body,
    ) -> impl Stream<Item = Result<Vec<ApiResponse>>> + 'a {
        let client = self.client.clone();
        let base_url = self.base_url.clone();
        let api_key = self.api_key.clone();

        async_stream::try_stream! {
            let json = serde_json::to_string(&body)?;

            let response = client
                .post(base_url.clone())
                .header("Content-Type", "application/json")
                .header("Authorization", format!("Bearer {}", api_key))
                .body(json)
                .send()
                .await
                .context("during request.")?;

            if !response.status().is_success() {
                Err(anyhow!("Bad Request {}", response.status()))?; // why no return here?
            }

            let mut stream = response.bytes_stream();

            while let Some(chunk_res) = stream.next().await {
                let chunk = chunk_res.context("Failed to get chunk")?;
                if let Ok(raw_response) = String::from_utf8(chunk.to_vec()) {

                    let filtered_response: Vec<ApiResponse> = raw_response
                        .lines()
                        .filter(|line| line.starts_with("data: ") && !line.contains("[DONE]"))
                        .map(|line| line.trim_start_matches("data: ").to_string())
                        .map(|line| {
                            serde_json::from_str::<ApiResponse>(line.as_str()).map_err(|e| anyhow!("Failed to deserialize json response. {}", e))
                        })
                        .collect::<Result<Vec<ApiResponse>>>()?;

                    yield filtered_response;

                } else {
                    Err(anyhow!("Failed to read chunk: {:?}", chunk))?; // why no return here?
                }
            }
        }
    }
}
