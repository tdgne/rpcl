use crossterm::{InputEvent, KeyEvent, ClearType, Attribute};
use number_prefix::{NumberPrefix, Standalone, Prefixed};

use crate::tui::app::App;
use crate::repository::{Repository, RepositoryStore};

pub struct PathList {
    pub pos: usize,
    pub offset: usize,
    pub path_scroll_amount: usize,
    pub height: usize,
}

pub enum Event {
    Open(Repository),
}

impl PathList {
    pub fn go_up(&mut self) {
        if self.pos > 0 {
            self.pos -= 1;
        } else if self.offset > 0 {
            self.offset -= 1;
        }
        self.path_scroll_amount = 0;
    }

    pub fn go_down(&mut self, list_len: usize) {
        if self.pos + 1 < self.height {
            self.pos += 1;
        } else if self.offset + self.pos + 1 < list_len {
            self.offset += 1;
        }
        self.path_scroll_amount = 0;
    }

    pub fn go_to_top(&mut self) {
        self.pos = 0;
        self.offset = 0;
        self.path_scroll_amount = 0;
    }

    pub fn go_to_bottom(&mut self, list_len: usize) {
        if list_len < self.height {
            self.pos = list_len - 1;
        } else {
            self.pos = self.height - 1;
            self.offset = list_len - self.height;
        }
        self.path_scroll_amount = 0;
    }

    pub fn draw(&self, repositories: &Vec<Repository>) -> crossterm::Result<()> {
        let terminal = crossterm::terminal();
        for i in 0..self.height {
            if let Some(repository) = repositories.get(self.offset + i) {
                if repository.size() > 0 {
                    self.render_repository(&repository, i == self.pos)?;
                } else {
                    terminal.clear(ClearType::CurrentLine)?;
                    terminal.write("\r\n")?;
                }
            } else {
                terminal.clear(ClearType::CurrentLine)?;
                terminal.write("\r\n")?;
            }
        }
        Ok(())
    }

    fn render_repository(&self, repository: &Repository, selected: bool) -> crossterm::Result<()> {
        let terminal = crossterm::terminal();
        terminal.clear(ClearType::CurrentLine)?;
        let size = repository.size();
        let (width, height) = terminal.size()?;
        let size_str = match NumberPrefix::binary(size as f64) {
            Standalone(bytes) => format!("{}", bytes),
            Prefixed(prefix, n) => format!("{:>6.1} {}B", n, prefix),
        };
        if selected {
            let path_str = scroll_line_if_needed(repository.path().to_string_lossy().to_string(), width as usize - 11, self.path_scroll_amount);
            terminal.write(Attribute::Reverse)?;
            terminal.write(format!("{:<11}{}\r\n", size_str, path_str))?;
            terminal.write(Attribute::Reset)?;
        } else {
            let path_str = scroll_line_if_needed(repository.path().to_string_lossy().to_string(), width as usize - 11, 0);
            terminal.write(format!("{:<11}{}\r\n", size_str, path_str))?;
        }
        Ok(())
    }

    fn get_selected_repository(&self, repositories: &RepositoryStore) -> Result<Repository, Box<dyn std::error::Error>> {
        let repositories = repositories.repositories_sorted()?;
        return Ok(repositories[self.pos + self.offset].clone());
    }

    pub fn input(&mut self, event: InputEvent, repositories: &RepositoryStore) -> Result<Option<Event>, Box<dyn std::error::Error>> {
        match event {
            InputEvent::Keyboard(k) => {
                match k {
                    KeyEvent::Char(c) => match c {
                        'j' => {
                            self.go_down(repositories.filtered_len()?);
                        },
                        'k' => {
                            self.go_up();
                        },
                        'g' => {
                            self.go_to_top();
                        },
                        'G' => {
                            self.go_to_bottom(repositories.filtered_len()?);
                        },
                        _ => {},
                    },
                    KeyEvent::Enter => {
                        return Ok(Some(Event::Open(self.get_selected_repository(repositories)?)));
                    },
                    KeyEvent::Up => {
                        self.go_up();
                    },
                    KeyEvent::Down => {
                        self.go_down(repositories.filtered_len()?);
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
