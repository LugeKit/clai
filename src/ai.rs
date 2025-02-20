use crate::config::Config;
use crate::parameter::Parameter;
use anyhow::{anyhow, Context};
use reqwest::blocking::Client;
use serde::{Deserialize, Serialize};
use std::env;
use std::time::Duration;

const VOLC_API: &'static str = "ark.cn-beijing.volces.com/api/v3";

pub struct Requester {
    client: Client,
    messages: Vec<Message>,
    model: String,
    base_url: String,
    api_key: String,
    timeout: Duration,
}

impl Requester {
    pub fn new(parameter: &Parameter, config: &Config) -> anyhow::Result<Requester> {
        Ok(Requester {
            client: Client::new(),
            messages: vec![Message {
                role: "system".to_string(),
                content: parameter.prompt.clone().unwrap_or(config.prompt.clone()),
            }],
            model: config.access_point.clone(),
            base_url: format!("https://{VOLC_API}/chat/completions"),
            api_key: env::var("LLM_API_KEY").context("failed to read env var `LLM_API_KEY`")?,
            timeout: Duration::from_secs(parameter.timeout.unwrap_or(config.timeout)),
        })
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

        if !response.status().is_success() {
            return Err(anyhow!("response code: {}", response.status()));
        }

        let body = response.text().context("fail to get http response")?;

        let result: Response =
            serde_json::from_str(body.as_str()).context("fail to unmarshal json")?;

        self.messages.push(result.choices[0].message.clone());

        Ok(result.choices[0].message.content.trim().to_string())
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
