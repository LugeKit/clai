use crate::config::Config;
use crate::parameter::Parameter;
use anyhow::{anyhow, Context};
use colored::Colorize;
use serde::{Deserialize, Serialize};
use std::env;
use std::time::Duration;

pub struct Requester {
    client: reqwest::Client,
    messages: Vec<Message>,
    model: String,
    base_url: String,
    api_key: String,
    timeout: Duration,
}

impl Requester {
    pub fn new(parameter: &Parameter, config: &Config) -> anyhow::Result<Requester> {
        let model = match &parameter.model {
            None => &config.models[&config.default_model],
            Some(model) => &config.models[model],
        };

        Ok(Requester {
            client: reqwest::Client::new(),
            messages: vec![Message {
                role: "system".to_string(),
                content: parameter.prompt.clone().unwrap_or(config.prompt.clone()),
                reasoning_content: None,
            }],
            model: model.access_point.clone(),
            base_url: format!(
                "https://{}/chat/completions",
                model.base_url.clone().unwrap_or(config.base_url.clone())
            ),
            api_key: env::var("LLM_API_KEY").context("failed to read env var `LLM_API_KEY`")?,
            timeout: Duration::from_secs(parameter.timeout.unwrap_or(config.timeout)),
        })
    }

    pub async fn request(&mut self, message: impl Into<String>) -> anyhow::Result<()> {
        let _message = Message {
            role: "user".to_string(),
            content: message.into(),
            reasoning_content: None,
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
            .await
            .context("fail to do request")?;

        if !response.status().is_success() {
            return Err(anyhow!("response code: {}", response.status()));
        }

        self.resolve_response(response).await?;

        Ok(())
    }

    async fn resolve_response(&mut self, resp: reqwest::Response) -> anyhow::Result<()> {
        let body = resp.text().await.context("fail to get http response")?;

        let result: Response =
            serde_json::from_str(body.as_str()).context("fail to unmarshal json")?;

        let _message = Message {
            role: result.choices[0].message.role.clone(),
            content: result.choices[0].message.content.clone(),
            reasoning_content: None,
        };

        if let Some(reasoning_content) = &result.choices[0].message.reasoning_content {
            println!("{} {}\n", "thinking:".blue().bold(), reasoning_content);
        }

        println!("{} {}\n", "answer:".green().bold(), _message.content.trim());
        self.messages.push(_message);
        Ok(())
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
    reasoning_content: Option<String>,
}
