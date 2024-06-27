use std::error::Error;
use openai_api_rs::v1::api::Client;
use openai_api_rs::v1::chat_completion::{ChatCompletionRequest, ChatCompletionMessage, MessageRole, Content};

const MODEL: &str = "gpt-3.5-turbo";
const MAX_TOKENS: i64 = 500;
const TEMPERATURE: f64 = 0.7;

pub fn generate_response(client: &Client, prompt: &str) -> Result<String, Box<dyn Error>> {
    let req = ChatCompletionRequest {
        model: MODEL.to_string(),
        messages: vec![ChatCompletionMessage {
            role: MessageRole::user,
            content: Content::Text(prompt.to_string()),
            name: None,
        }],
        temperature: Some(TEMPERATURE),
        top_p: None,
        n: None,
        response_format: None,
        stream: None,
        stop: None,
        max_tokens: Some(MAX_TOKENS),
        presence_penalty: None,
        frequency_penalty: None,
        logit_bias: None,
        user: None,
        seed: None,
        tools: None,
        tool_choice: None,
    };

    let result = client.chat_completion(req)?;
    result.choices
        .get(0)
        .and_then(|choice| choice.message.content.as_ref())
        .map(|content| content.to_string())
        .ok_or_else(|| "No response generated".into())
}