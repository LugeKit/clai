use anyhow::Context;
use clap::Parser;
use std::io::{stdin, Read, Write};
use std::process::exit;
use std::{env, io};

mod ai;
mod config;
mod parameter;

fn main() {
    if let Err(e) = main_process() {
        println!("error: {}", e);
        exit(1);
    }
    exit(0);
}

fn main_process() -> anyhow::Result<()> {
    let param = parameter::Parameter::parse();
    let config = config::Config::new();

    let prompt = param.prompt.unwrap_or(config.prompt);
    let timeout = param.timeout.unwrap_or(config.timeout);
    let api_key = env::var("LLM_API_KEY").context("failed to get env var `LLM_API_KEY`")?;

    let mut requester = ai::Requester::new(prompt, timeout, api_key);
    let mut first_answer = true;

    if param.interactive {
        loop {
            if !first_answer {
                println!("--------------------------");
            }
            first_answer = false;

            print!("question: ");
            io::stdout().flush().context("failed to flush stdout")?;

            let mut query = String::new();
            stdin()
                .read_line(&mut query)
                .context("failed to read from input")?;
            if query.trim().to_lowercase() == "exit" {
                break;
            }

            let result = requester.request(query);
            println!("answer: {}\n", result.unwrap_or_else(|e| { e.to_string() }));
        }
        return Ok(());
    }

    let mut query = String::new();
    match param.query {
        None => {
            stdin()
                .read_to_string(&mut query)
                .context("failed to read from stdin")?;
        }
        Some(_query) => {
            query = _query;
        }
    }

    let result = requester.request(query);
    println!("answer: {}\n", result.unwrap_or_else(|e| { e.to_string() }));
    Ok(())
}
