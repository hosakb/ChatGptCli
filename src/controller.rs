use std::io;
use std::io::Write;

use anyhow::{anyhow, Context as AnyhowContext, Result};
use tokio_stream::StreamExt;

use crate::api::GptClient;
use crate::model::{Body, Model};

#[derive(Clone)]
pub struct Context {
    api_key: String,
    base_url: String,
}

impl Context {
    pub fn new(api_key: String, base_url: String) -> Context {
        Context { api_key, base_url }
    }
}

pub struct Controller {
    context: Context,
    client: GptClient,
}

impl Controller {
    pub fn new(context: Context) -> Controller {
        Controller {
            context: context.clone(),
            client: GptClient::new(context.base_url, context.api_key),
        }
    }

    pub async fn new_chat(&self) -> Result<()> {
        let mut prompt = String::new();

        io::stdin()
            .read_line(&mut prompt)
            .context("Failed to read user input")?;

        let body = Body::new(Model::Gpt4o, prompt);

        let mut stream = Box::pin(self.client.fetch_response_as_stream(body));

        while let Some(chunk) = stream.next().await {
            match chunk {
                Ok(content) => {
                    print!("{content}");
                    io::stdout().flush().expect("Failed to flush stdout");
                }
                Err(e) => Err(anyhow!("{}", e))?,
            }
        }

        Ok(())
    }

    // pub fn respond_to_chat() -> String {

    // }

    // pub fn delegate_response() -> String {

    // }
}
