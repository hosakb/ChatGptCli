use serde::{Deserialize, Serialize};

#[derive(Deserialize, Debug)]
pub struct ApiResponse {
    id: String,
    object: String,
    created: i32,
    model: String,
    system_fingerprint: String,
    pub choices: Vec<Choice>,
}
#[derive(Deserialize, Debug)]
pub struct Choice {
    index: u32,
    pub delta: Delta,
    logprobs: Option<String>,
    finish_reason: Option<String>,
}
#[derive(Deserialize, Debug)]
pub struct Delta {
    pub content: Option<String>,
}

#[derive(Serialize, Clone)]
// #[serde(rename_all = "kebab-case")]
pub enum Model {
    Gpt4oMini,
    #[serde(rename = "gpt-4o")]
    Gpt4o,
    GptO1,
}

#[derive(Serialize, Clone)]
#[serde(rename_all = "lowercase")]
enum Role {
    USER,
    ASSISTANT,
}

#[derive(Serialize, Clone)]
pub struct Message {
    role: Role,
    content: String,
}

impl Message {
    pub fn new(role: Role, content: String) -> Message {
        Message { role, content }
    }
}

#[derive(Serialize, Clone)]
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
