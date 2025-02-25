use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, about, long_about=None)]
pub struct Parameter {
    pub query: Option<String>,

    #[arg(short = 'p', long = "prompt")]
    pub prompt: Option<String>,

    #[arg(short = 't', long = "timeout")]
    #[clap(help = "Set request timeout seconds")]
    pub timeout: Option<u64>,

    #[arg(short = 'i', long = "interactive", default_value_t = false)]
    pub interactive: bool,

    #[arg(short = 'm', long = "model")]
    pub model: Option<String>,
}
