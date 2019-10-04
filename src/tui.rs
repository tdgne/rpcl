use std::error::Error;
use std::thread;
use std::time::Duration;
use std::sync::mpsc::Receiver;
use crossterm::{style, Attribute, Terminal, RawScreen, input, InputEvent, KeyEvent, AlternateScreen, ClearType, Color, Crossterm, Styler};
use number_prefix::{NumberPrefix, Standalone, Prefixed};

use crate::collector;
use crate::repository::*;

pub struct PathList {
    pub pos: usize,
    pub offset: usize,
    pub path_scroll_amount: usize,
}

impl PathList {
    fn go_up(&mut self) {
        if self.pos > 0 {
            self.pos -= 1;
        } else if self.offset > 0 {
            self.offset -= 1;
        }
        self.path_scroll_amount = 0;
    }

    fn go_down(&mut self, list_height: usize) {
        if self.pos + 1 < list_height {
            self.pos += 1;
        } else {
            self.offset += 1;
        }
        self.path_scroll_amount = 0;
    }

    fn draw(&self) {
        // TODO: Refactor
    }
}

pub struct App {
    pub repositories: RepositoryStore,
    pub root_path: String,
    pub spinner_phase: usize,
    pub path_list: PathList,
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

fn render_repository(app: &App, repository: &Repository, terminal: &Terminal, selected: bool) -> crossterm::Result<()> {
    terminal.clear(ClearType::CurrentLine)?;
    let size = repository.size();
    let (width, height) = terminal.size()?;
    let size_str = match NumberPrefix::binary(size as f64) {
        Standalone(bytes) => format!("{}", bytes),
        Prefixed(prefix, n) => format!("{:>5.1} {}B", n, prefix),
    };
    if selected {
        let path_str = scroll_line_if_needed(repository.path().to_string_lossy().to_string(), width as usize - 10, app.path_list.path_scroll_amount);
        terminal.write(Attribute::Reverse)?;
        terminal.write(format!("{:<10}{}\r\n", size_str, path_str))?;
        terminal.write(Attribute::Reset)?;
    } else {
        let path_str = scroll_line_if_needed(repository.path().to_string_lossy().to_string(), width as usize - 10, 0);
        terminal.write(format!("{:<10}{}\r\n", size_str, path_str))?;
    }
    Ok(())
}

pub fn run_tui(
    mut app: App,
    collector_rx: Receiver<collector::Event>,
    spinner_rx: Receiver<()>
) -> Result<(), Box<dyn Error>> {
    let _raw = RawScreen::into_raw_mode()?;
    let crossterm = Crossterm::new();
    let terminal = crossterm.terminal();
    terminal.clear(ClearType::All)?;
    let cursor = crossterm.cursor();
    cursor.hide()?;
    let (width, height) = terminal.size()?;

    let input = input();
    let mut stdin = input.read_async();

    let spinner_strs = ["◡◡", "⊙⊙", "◠◠", "⊙⊙"];
    let mut done = false;

    

    loop {
        let mut render = true;
        if let Some(event) = stdin.next() {
            match event {
                InputEvent::Keyboard(k) => {
                    match k {
                        KeyEvent::Char(c) => match c {
                            'q' => {
                                break;
                            },
                            'j' => {
                                app.path_list.go_down(height as usize - 2);
                            },
                            'k' => {
                                app.path_list.go_up();
                            },
                            _ => {},
                        },
                        KeyEvent::Ctrl('c') => {
                            break;
                        },
                        KeyEvent::Up => {
                            app.path_list.go_up();
                        },
                        KeyEvent::Down => {
                            app.path_list.go_down(height as usize - 2);
                        }
                        _ => {},
                    }
                }
                _ => {},
            }
        }
        if let Ok(event) = collector_rx.try_recv() {
            match event {
                collector::Event::Update => {
                    render = true;
                },
                collector::Event::Done => {
                    done = true;
                    render = true;
                }
            }
        }
        if let Ok(_) = spinner_rx.try_recv() {
            render = true;
            app.spinner_phase += 1;
            app.spinner_phase %= 4;
        }
        if render {
            cursor.goto(0, 0)?;
            terminal.clear(ClearType::CurrentLine)?;
            terminal.write("q: Quit  j,k: Move\r\n")?;
            let list_height = height as usize - 2;
            let repositories = app.repositories.repositories_sorted()?;
            for i in 0..list_height {
                if let Some(repository) = repositories.get(app.path_list.offset + i) {
                    if repository.size() > 0 {
                        render_repository(&app, &repository, &terminal, i == app.path_list.pos)?;
                    }
                }
            }
            terminal.clear(ClearType::CurrentLine)?;
            if done {
                terminal.write(format!("Done."))?;
            } else {
                terminal.write(format!("{} Searching under {}", spinner_strs[app.spinner_phase], app.root_path))?;
            }
        }
        thread::sleep(Duration::from_millis(67));
        if app.path_list.path_scroll_amount < 1000 {
            app.path_list.path_scroll_amount += 1;
        }
    }

    cursor.show()?;

    Ok(())
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
