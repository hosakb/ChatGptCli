mod api;
mod cli;
mod controller;
mod model;

use std::env;

use anyhow::{self, Context as AnyhowContext};

use controller::{Context, Controller};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenv::dotenv().ok();

    let api_key = env::var("OPENAI_API_KEY").context("Failed to read api key env.")?;
    let base_url = env::var("BASE_URL").context("Failed to read base url env.")?;

    let context = Context::new(api_key, base_url);
    let controller = Controller::new(context);

    controller.new_chat().await.context("During new chat.")?;

    Ok(())
}
