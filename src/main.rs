use clap::Parser;
use std::io::Error;

use grrs::config::TypingConfig;
use grrs::Typing;
fn main() -> Result<(), Error> {
    let config = TypingConfig::parse();
    let mut typing = Typing::new(config).unwrap();

    loop {
        if let Ok((false, _)) = typing.run_typing() {
            break;
        }
    }
    Ok(())
}
