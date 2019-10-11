use crossterm::{KeyEvent, InputEvent};

use crate::repository::Repository;
use crate::tui::list::List;

pub struct Details {
    pub list: List,
}

pub enum Event {
    Close,
}

impl Details {
    pub fn input(&mut self, event: InputEvent, repository: &Repository) -> Result<Option<Event>, Box<dyn std::error::Error>> {
        self.list.input(event.clone(), 1 + repository.ignored_path_infos().len());
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
        let mut strings = Vec::new();
        strings.push(format!("{}\r\n", repository.path().to_string_lossy()));
        for ignored_path_info in repository.ignored_path_infos().iter() {
            strings.push(format!("{}\r\n", ignored_path_info.path().to_string_lossy()));
        }
        self.list.draw(&strings)?;
        Ok(())
    }
}
