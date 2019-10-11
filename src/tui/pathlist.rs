use crossterm::{InputEvent, KeyEvent, ClearType, Attribute};
use number_prefix::{NumberPrefix, Standalone, Prefixed};

use crate::tui::list::List;
use crate::repository::{Repository, RepositoryStore};


pub struct PathList {
    pub list: List,
    pub path_scroll_amount: usize,
}

pub enum Event {
    Open(Repository),
}

impl PathList {
    pub fn draw(&self, repositories: &Vec<Repository>) -> crossterm::Result<()> {
        self.list.draw(&repositories
                       .iter()
                       .filter(|r| r.size() != 0)
                       .enumerate()
                       .flat_map(|(i, r)| self.render_repository(r, i == self.list.offset + self.list.pos))
                       .collect::<Vec<_>>())?;
        Ok(())
    }

    fn render_repository(&self, repository: &Repository, selected: bool) -> Result<String, Box<dyn std::error::Error>> {
        let terminal = crossterm::terminal();
        let size = repository.size();
        let (width, _height) = terminal.size()?;
        let size_str = match NumberPrefix::binary(size as f64) {
            Standalone(bytes) => format!("{}", bytes),
            Prefixed(prefix, n) => format!("{:>6.1} {}B", n, prefix),
        };
        Ok(if selected {
            let path_str = scroll_line_if_needed(repository.path().to_string_lossy().to_string(), width as usize - 11, self.path_scroll_amount);
            format!("{:<11}{}\r\n", size_str, path_str)
        } else {
            let path_str = scroll_line_if_needed(repository.path().to_string_lossy().to_string(), width as usize - 11, 0);
            format!("{:<11}{}\r\n", size_str, path_str)
        })
    }

    fn get_selected_repository(&self, repositories: &RepositoryStore) -> Result<Repository, Box<dyn std::error::Error>> {
        let repositories = repositories.repositories_sorted()?;
        return Ok(repositories[self.list.pos + self.list.offset].clone());
    }

    pub fn input(&mut self, event: InputEvent, repositories: &RepositoryStore) -> Result<Option<Event>, Box<dyn std::error::Error>> {
        self.list.input(event.clone(), repositories.filtered_len()?);
        match event {
            InputEvent::Keyboard(k) => {
                match k {
                    KeyEvent::Char(c) => match c {
                        'j' => {
                            self.path_scroll_amount = 0;
                        },
                        'k' => {
                            self.path_scroll_amount = 0;
                        },
                        'g' => {
                            self.path_scroll_amount = 0;
                        },
                        'G' => {
                            self.path_scroll_amount = 0;
                        },
                        _ => {},
                    },
                    KeyEvent::Enter => {
                        return Ok(Some(Event::Open(self.get_selected_repository(repositories)?)));
                    },
                    KeyEvent::Up => {
                        self.path_scroll_amount = 0;
                    },
                    KeyEvent::Down => {
                        self.path_scroll_amount = 0;
                    }
                    _ => {},
                }
            }
            _ => {},
        }
        Ok(None)
    }
}

fn scroll_line_if_needed(mut line: String, width: usize, path_scroll_amount: usize) -> String {
    if line.len() < width {
        return line;
    }
    if (line.len() as isize) - (path_scroll_amount as isize) < width as isize {
        return line.split_off(line.len() - width);
    }
    let mut line = line.split_off(path_scroll_amount);
    line.split_off(width);
    line
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_scroll_line_if_needed() {
        assert_eq!(scroll_line_if_needed("".to_string(), 3, 0), "");
        assert_eq!(scroll_line_if_needed("".to_string(), 3, 2), "");
        assert_eq!(scroll_line_if_needed("abcde".to_string(), 3, 100), "cde");
        assert_eq!(scroll_line_if_needed("abc".to_string(), 3, 1), "abc");
        assert_eq!(scroll_line_if_needed("abcde".to_string(), 3, 0), "abc".to_string());
        assert_eq!(scroll_line_if_needed("abcde".to_string(), 3, 1), "bcd".to_string());
        assert_eq!(scroll_line_if_needed("abcde".to_string(), 3, 2), "cde".to_string());
        assert_eq!(scroll_line_if_needed("abcde".to_string(), 3, 3), "cde".to_string());
    }
}
