mod api;

use std::env;

use anyhow::{self, Context};

use api::{Body, GptClient, Model};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenv::dotenv().ok();

    let api_key = env::var("OPENAI_API_KEY").context("Failed to read api key env.")?;
    let base_url = env::var("BASE_URL").context("Failed to read base url env.")?;

    let client = GptClient::new(base_url, api_key);

    let body = Body::new(Model::Gpt4o, String::from("Write a very long text."));

    client.send_prompt(body).await?;

    Ok(())
}
