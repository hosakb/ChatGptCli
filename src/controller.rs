use std::io;
use std::io::Write;

use anyhow::{anyhow, Context as AnyhowContext, Result};
use tokio_stream::StreamExt;

use crate::api::GptClient;
use crate::model::{Body, Message, Model, Role};

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
    client: GptClient,
}

impl Controller {
    pub fn new(context: Context) -> Controller {
        Controller {
            client: GptClient::new(context.base_url, context.api_key),
        }
    }

    pub async fn chat(&self) -> Result<()> {
        let mut messages: Vec<Message> = Vec::new();

        loop {
            let mut prompt = String::new();
            println!("User: ");
            io::stdin()
                .read_line(&mut prompt)
                .context("Failed to read user input")?;

            let message = Message::new(Role::User, prompt);
            messages.push(message);

            let body = Body::new(Model::Gpt4o, messages.clone());

            let mut stream = Box::pin(self.client.fetch_response_as_stream(body));

            let mut response_message = String::new();
            println!();
            println!("Assistant: ");
            while let Some(chunk) = stream.next().await {
                match chunk {
                    Ok(responses) => {
                        for res in responses {
                            //TODO: persist_response()
                            for choice in res.choices {
                                if let Some(choice) = choice.delta.content {
                                    let token = choice.replace("\"", "");
                                    response_message += &token;
                                    print!("{token}");
                                    io::stdout().flush().context("Failed to flush stdout")?;
                                }
                            }
                        }
                    }
                    Err(e) => Err(anyhow!("{}", e))?,
                }
            }
            println!();
            messages.push(Message::new(Role::Assistant, response_message));
        }
    }
}
