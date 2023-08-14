pub mod config;
pub mod results;
pub mod textrand;
pub mod tui;
pub mod wordlists;

use crossterm::event::{self, Event};
use std::{io::Error, result::Result};

use crate::tui::TypingTui;
use config::TypingConfig;
use results::TypingResults;
use textrand::WordSelector;
use wordlists::get_word_list;

#[derive(Debug)]
pub struct TypingError {
    pub err: String,
}

impl From<Error> for TypingError {
    fn from(error: Error) -> Self {
        TypingError {
            err: error.to_string(),
        }
    }
}

pub struct Typing {
    pub words: Vec<String>,
    pub tui: TypingTui,
    pub config: TypingConfig,
    pub word_selector: WordSelector,
}

impl<'a> Typing {
    pub fn new(config: TypingConfig) -> Result<Self, TypingError> {
        if let Some(word_lists) = get_word_list(&config.wordfile) {
            let mut typing = Self {
                words: Vec::new(),
                word_selector: WordSelector::from_string(word_lists),
                tui: TypingTui::new().unwrap(),
                config,
            };
            typing.restart()?;

            Ok(typing)
        } else {
            Err(TypingError {
                err: String::from("not found file"),
            })
        }
    }

    pub fn restart(&mut self) -> Result<(), TypingError> {
        self.tui.reset_screen()?;
        self.tui.draw_border()?;

        self.words = self.word_selector.new_words(self.config.num_words);

        self.tui.dispaly_words(&self.words)?;
        Ok(())
    }

    pub fn run_typing(&mut self) -> Result<(bool, TypingResults), Error> {
        match event::read()? {
            Event::Key(event) => {
                if event
                    == event::KeyEvent::new(event::KeyCode::Char('c'), event::KeyModifiers::CONTROL)
                {
                    return Ok((false, None));
                }
                Ok((true, None))
            }
            _ => Ok((true, None)),
        }
    }
}
