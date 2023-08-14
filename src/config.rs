use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about)]
pub struct TypingConfig {
    #[arg(short, long, default_value_t = String::from("top250"))]
    pub wordfile: String,

    #[arg(short, long, default_value_t = 30)]
    pub num_words: usize,
}
