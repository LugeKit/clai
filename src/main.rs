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

    let timeout = if param.timeout.is_none() {
        config.timeout
    } else {
        param.timeout.expect("")
    };

    let mut requester = ai::Requester::new(prompt, timeout);
    let mut first_answer = true;

    if param.interactive {
        loop {
            if !first_answer {
                println!("--------------------------");
            }
            first_answer = false;

            print!("question: ");
            io::stdout().flush().expect("failed to flush stdout");

            let mut query = String::new();
            stdin().read_line(&mut query).expect("failed to readline");
            if query.trim().to_lowercase() == "exit" {
                break;
            }

            let result = requester.request(query);
            println!("answer: {}\n", result.unwrap_or_else(|e| { e.to_string() }));
        }
        exit(0)
    }

    let mut query = String::new();
    match param.query {
        None => {
            stdin()
                .read_to_string(&mut query)
                .expect("failed to read from stdin");
        }
        Some(_query) => {
            query = _query;
        }
    }

    let result = requester.request(query);
    println!("answer: {}\n", result.unwrap_or_else(|e| { e.to_string() }));

    exit(0);
}
