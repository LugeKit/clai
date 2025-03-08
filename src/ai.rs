use crate::config::Config;
use crate::parameter::Parameter;
use anyhow::{anyhow, Context};
use colored::Colorize;
use serde::{Deserialize, Serialize};
use std::env;
use std::time::Duration;
use tokio_stream::StreamExt;

pub struct Requester {
    client: reqwest::Client,
    messages: Vec<Message>,
    model: String,
    base_url: String,
    api_key: String,
    timeout: Duration,
    stream: bool,
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
            stream: parameter.stream,
        })
    }

    pub async fn request(&mut self, message: impl Into<String>) -> anyhow::Result<()> {
        let _message = Message {
            role: "user".to_string(),
            content: message.into(),
            reasoning_content: None,
        };

        self.messages.push(_message);

        let data = Request::new(&self.model, &self.messages, self.stream);

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

        if self.stream {
            self.resolve_response_streaming(response).await?;
        } else {
            self.resolve_response(response).await?;
        }

        Ok(())
    }

    async fn resolve_response_streaming(&mut self, response: reqwest::Response) -> anyhow::Result<()> {
        let mut bytes_stream = response.bytes_stream();
        let mut _message = Message {
            role: String::from(""),
            content: String::from(""),
            reasoning_content: None,
        };
        let mut skin = termimad::MadSkin::new();

        // 0: not start
        // 1: thinking
        // 2: finish
        let mut thinking_part = 0;

        while let Some(chunk) = bytes_stream.next().await {
            let response_str = String::from_utf8(chunk?.to_vec()).context("fail to convert bytes_stream to str")?;
            for line in response_str.lines() {
                let data = line.trim().trim_start_matches("data: ");

                if data.is_empty() {
                    continue;
                }

                if data == "[DONE]" {
                    break;
                }

                let result: StreamResponse = serde_json::from_str(data)?;
                let current_message = &result.choices[0].delta;

                if let Some(reasoning_content) = &current_message.reasoning_content {
                    if thinking_part == 0 {
                        print!("{}", "thinking: ".blue().bold());
                        thinking_part = 1;
                    }

                    print!("{}", skin.text(reasoning_content).to_string().trim());
                    continue;
                }

                if thinking_part <= 1 {
                    if thinking_part == 1 {
                        println!();
                    }
                    print!("{}", "answer: ".green().bold());
                    thinking_part = 2;
                }

                let content = &current_message.content;
                if content.is_empty() {
                    continue;
                }

                let printed_str = skin.text(content).to_string();
                print!("{}", printed_str.trim());
                _message.content.push_str(&content);
                _message.role = current_message.role.clone();
            }
        }
        println!();
        self.messages.push(_message);
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

        let mut skin = termimad::MadSkin::new();

        if let Some(reasoning_content) = &result.choices[0].message.reasoning_content {
            println!(
                "{} {}",
                "thinking:".blue().bold(),
                skin.text(reasoning_content),
            );
        }

        println!(
            "{} {}",
            "answer:".green().bold(),
            skin.text(_message.content.trim())
        );
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
    fn new<'a>(model: &'a String, messages: &'a Vec<Message>, stream: bool) -> Request<'a> {
        Request {
            model,
            stream,
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

#[derive(Deserialize, Debug)]
struct StreamResponse {
    choices: Vec<StreamChoice>,
}

#[derive(Deserialize, Debug)]
struct StreamChoice {
    delta: StreamMessage,
}

#[derive(Deserialize, Debug)]
struct StreamMessage {
    role: String,
    content: String,
    reasoning_content: Option<String>,
}