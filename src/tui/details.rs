use std::path::PathBuf;

use crossterm::{KeyEvent, InputEvent};
use number_prefix::{NumberPrefix, Standalone, Prefixed};

use crate::repository::Repository;
use crate::tui::list::List;

pub struct Details {
    pub list: List,
}

pub enum Event {
    Close,
    DeleteAll,
    Delete(PathBuf),
}

fn size_str(size: u64) -> String {
    match NumberPrefix::binary(size as f64) {
        Standalone(bytes) => format!("{:>6}   B", bytes),
        Prefixed(prefix, n) => format!("{:>6.1} {}B", n, prefix),
    }
}

impl Details {
    pub fn input(&mut self, event: InputEvent, repository: &Repository) -> Result<Option<Event>, Box<dyn std::error::Error>> {
        self.list.input(event.clone(), 1 + repository.ignored_path_infos().len());
        match event {
            InputEvent::Keyboard(k) => {
                match k {
                    KeyEvent::Char(c) => {
                        match c {
                            'd' => {
                                if self.list.pos == 0 {
                                    return Ok(Some(Event::DeleteAll));
                                } else {
                                    return Ok(Some(Event::Delete(repository.ignored_path_infos()[self.list.pos - 1].path().to_path_buf())));
                                }
                            },
                            _ => {
                            },
                        }
                    },
                    KeyEvent::Enter => {
                        self.list.pos = 0;
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
        strings.push(format!("{:<11}{}\r\n", size_str(repository.size()), repository.path().to_string_lossy()));
        for ignored_path_info in repository.ignored_path_infos().iter() {
            if ignored_path_info.size() > 0 {
                strings.push(format!("    {:<11}{}\r\n", size_str(ignored_path_info.size()), ignored_path_info.path().to_string_lossy()));
            }
        }
        self.list.draw(&strings)?;
        Ok(())
    }
}
