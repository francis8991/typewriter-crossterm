use crate::TypingError;
use crossterm::{
    cursor::MoveTo,
    execute, queue,
    style::Print,
    terminal::{enable_raw_mode, Clear, ClearType},
};
use std::io::{stdout, Stdout};
use tui::{
    backend::CrosstermBackend,
    widgets::{Block, Borders},
    Terminal,
};
type MaybeError<T = ()> = Result<T, TypingError>;
pub struct TypingTui {
    pub stdout: Stdout,
}

impl TypingTui {
    pub fn new() -> MaybeError<Self> {
        enable_raw_mode().unwrap();
        let stdout = stdout();

        Ok(Self { stdout })
    }

    pub fn draw_border(&mut self) -> MaybeError {
        let handle = &mut self.stdout.lock();
        let backend = CrosstermBackend::new(handle);
        let mut terminal = Terminal::new(backend)?;

        terminal.draw(|f| {
            let size = f.size();
            let block = Block::default().title("Typing").borders(Borders::ALL);
            f.render_widget(block, size);
        })?;

        Ok(())
    }

    pub fn reset_screen(&mut self) -> MaybeError {
        let handle = &mut self.stdout.lock();
        execute!(handle, Clear(ClearType::All))?;
        Ok(())
    }

    pub fn dispaly_words(&mut self, words: &Vec<String>) -> MaybeError {
        let mut word_str = String::new();
        for word in words {
            word_str += &word[..];
        }
        let (width, height) = crossterm::terminal::size()?;
        let x = width / 2;
        let y = height / 2;
        queue!(self.stdout, MoveTo(x, y))?;
        queue!(self.stdout, Print(&word_str))?;
        execute!(self.stdout)?;

        Ok(())
    }
}

impl Drop for TypingTui {
    /// Resets terminal.
    ///
    /// Clears screen and sets the cursor to a non-blinking block.
    ///
    /// TODO: reset cursor to whatever it was before Toipe was started.
    fn drop(&mut self) {
        let _ = &mut self.reset_screen();
    }
}
