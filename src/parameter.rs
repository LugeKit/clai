use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, about, long_about=None)]
pub struct Parameter {
    pub query: Option<String>,

    #[arg(short = 'p', long = "prompt")]
    pub prompt: Option<String>,
}
