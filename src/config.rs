use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, about, long_about=None)]
pub struct Config {
    pub query: Option<String>,

    #[arg(
        short = 'p',
        long = "prompt",
        default_value = "你是一个优秀的程序员，现在用简短的回答回复我的问题，在没有经过要求的前提下，不要详细解释你的回答。"
    )]
    pub prompt: String,
}
