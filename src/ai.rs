use anyhow::Context;
use reqwest::blocking::Client;
use serde::{Deserialize, Serialize};
use std::time::Duration;

const DOUBAO_API: &'static str = "ark.cn-beijing.volces.com/api/v3";

pub struct Requester {
    client: Client,
    messages: Vec<Message>,
    model: String,
    base_url: String,
    api_key: String,
    timeout: Duration,
}

impl Requester {
    pub fn new(prompt: String, timeout: u64, api_key: String) -> Requester {
        Requester {
            client: Client::new(),
            messages: vec![Message {
                role: "system".to_string(),
                content: prompt,
            }],
            model: "ep-20250213110005-vwjgt".to_string(),
            base_url: format!("https://{DOUBAO_API}/chat/completions"),
            api_key,
            timeout: Duration::from_secs(timeout),
        }
    }

    pub fn request(&mut self, message: impl Into<String>) -> anyhow::Result<String> {
        let _message = Message {
            role: "user".to_string(),
            content: message.into(),
        };

        self.messages.push(_message);

        let data = Request::new(&self.model, &self.messages);

        let api_key = self.api_key.as_str();

        let response = self
            .client
            .post(&self.base_url)
            .header("Content-Type", "application/json")
            .header("Authorization", format!("Bearer {api_key}"))
            .json(&data)
            .timeout(self.timeout)
            .send()
            .context("fail to do request")?;

        let body = response.text().context("fail to get http response")?;
        let result: Response =
            serde_json::from_str(body.as_str()).context("fail to unmarshal json")?;

        self.messages.push(result.choices[0].message.clone());

        Ok(result.choices[0].message.content.clone())
    }
}

#[derive(Serialize)]
struct Request<'a> {
    model: &'a String,
    stream: bool,
    messages: &'a Vec<Message>,
    temperature: i32,
}

impl Request<'_> {
    fn new<'a>(model: &'a String, messages: &'a Vec<Message>) -> Request<'a> {
        Request {
            model,
            stream: false,
            messages,
            temperature: 0,
        }
    }
}

#[derive(Deserialize, Debug)]
struct Response {
    choices: Vec<Choice>,
}

#[derive(Deserialize, Debug)]
struct Choice {
    message: Message,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct Message {
    role: String,
    content: String,
}
