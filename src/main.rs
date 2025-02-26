use anyhow::Context;
use clap::Parser;
use colored::Colorize;
use std::io;
use std::io::{stdin, Read, Write};
use std::process::exit;

mod ai;
mod config;
mod parameter;

#[tokio::main]
async fn main() {
    if let Err(e) = main_process().await {
        println!("error: {}", e);
        exit(1);
    }
    exit(0);
}

async fn main_process() -> anyhow::Result<()> {
    let param = parameter::Parameter::parse();
    let config = config::Config::new();

    let mut requester = ai::Requester::new(&param, &config)?;
    let mut first_answer = true;

    if param.interactive {
        loop {
            if !first_answer {
                println!("--------------------------");
            }
            first_answer = false;

            print!("{} ", "question:".yellow());
            io::stdout().flush().context("failed to flush stdout")?;

            let mut query = String::new();
            stdin()
                .read_line(&mut query)
                .context("failed to read from input")?;
            query = query.trim().to_string();
            if query.to_lowercase() == "exit" {
                break;
            }

            let result = requester.request(query).await;
            if let Err(err) = result {
                println!("{} {}\n", "error:".red().bold(), err);
            }
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

    let result = requester.request(query).await;
    if let Err(err) = result {
        println!("{} {}\n", "error:".red().bold(), err);
    }
    Ok(())
}
