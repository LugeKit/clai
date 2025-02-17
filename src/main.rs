use clap::Parser;
use std::io;
use std::io::{stdin, Read, Write};
use std::process::exit;

mod ai;
mod config;
mod parameter;

fn main() {
    let param = parameter::Parameter::parse();
    let config = config::Config::new();
    let prompt = if param.prompt.is_none() {
        config.prompt
    } else {
        param.prompt.expect("")
    };

    println!("prompt: {}", prompt);

    let mut requester = ai::Requester::new(prompt);
    let mut first_answer = true;

    match param.query {
        None => loop {
            if !first_answer {
                println!("--------------------------");
            }

            first_answer = false;
            let mut query = String::new();
            print!("question: ");
            io::stdout().flush().expect("failed to flush stdout");
            stdin().read_line(&mut query).expect("failed to readline");
            if query.trim().to_lowercase() == "exit" {
                break;
            }

            let result = requester.request(query);
            println!("answer: {}\n", result.unwrap_or_else(|e| { e.to_string() }));
        },
        Some(query) => {
            let result = requester.request(query);
            println!("answer: {}\n", result.unwrap_or_else(|e| { e.to_string() }));
        }
    }

    exit(0);
}
