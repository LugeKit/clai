use clap::Parser;
use std::io;
use std::io::{stdin, Read, Write};
use std::process::exit;

mod ai;
mod config;

fn main() {
    let config = config::Config::parse();
    let mut requester = ai::Requester::new(config.prompt);

    match config.query {
        None => loop {
            let mut query = String::new();
            print!("提问: ");
            io::stdout().flush().expect("failed to flush stdout");
            stdin().read_line(&mut query).expect("failed to readline");
            if query.trim().to_lowercase() == "exit" {
                break;
            }

            let result = requester.request(query);
            println!("回答: {}\n", result.unwrap_or_else(|e| { e.to_string() }));
        },
        Some(query) => {
            let result = requester.request(query);
            println!("回答: {}\n", result.unwrap_or_else(|e| { e.to_string() }));
        }
    }

    exit(0);
}
