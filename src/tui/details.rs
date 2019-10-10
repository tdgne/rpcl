use crossterm::{KeyEvent, InputEvent, ClearType};

use crate::repository::Repository;

pub struct Details {
    pub height: usize,
}

pub enum Event {
    Close,
}

impl Details {
    pub fn input(&mut self, event: InputEvent) -> Result<Option<Event>, Box<dyn std::error::Error>> {
        match event {
            InputEvent::Keyboard(k) => {
                match k {
                    KeyEvent::Enter => {
                        return Ok(Some(Event::Close));
                    },
                    _ => {},
                }
            },
            _ => {},
        }
        Ok(None)
    }
     
    pub fn draw(&self, repository: Repository) -> crossterm::Result<()> {
        let terminal = crossterm::terminal();
        terminal.clear(ClearType::CurrentLine)?;
        terminal.write(format!("{}\r\n", repository.path().to_string_lossy()))?;
        for i in 1..self.height {
            terminal.clear(ClearType::CurrentLine)?;
            terminal.write("\r\n")?;
        }
        Ok(())
    }
}