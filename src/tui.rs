use std::error::Error;
use std::thread;
use std::time::Duration;
use std::sync::mpsc::Receiver;
use crossterm::{style, Attribute, RawScreen, input, InputEvent, KeyEvent, AlternateScreen, ClearType, Color, Crossterm, Styler};
use number_prefix::{NumberPrefix, Standalone, Prefixed};

use crate::collector;
use crate::repository::*;

pub struct App {
    pub repositories: RepositoryStore,
    pub root_path: String,
    pub spinner_phase: usize,
    pub y_pos: usize,
    pub path_scroll_amount: usize,
}

fn scroll_line_if_needed(mut line: String, width: usize, scroll_amount: usize) -> String {
    if line.len() < width {
        return line;
    }
    if (line.len() as isize) - (scroll_amount as isize) < width as isize {
        return line.split_off(line.len() - width);
    }
    let mut line = line.split_off(scroll_amount);
    line.split_off(width);
    line
}

pub fn run_tui(
    mut app: App,
    collector_rx: Receiver<collector::Event>,
    spinner_rx: Receiver<()>
) -> Result<(), Box<dyn Error>> {
    let _alt = AlternateScreen::to_alternate(true)?;
    let _raw = RawScreen::into_raw_mode()?;
    let crossterm = Crossterm::new();
    let terminal = crossterm.terminal();
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
                                app.y_pos += 1;
                                app.path_scroll_amount = 0;
                            },
                            'k' => {
                                if app.y_pos > 0 {
                                    app.y_pos -= 1;
                                    app.path_scroll_amount = 0;
                                }
                            },
                            _ => {},
                        },
                        KeyEvent::Ctrl('c') => {
                            break;
                        },
                        KeyEvent::Up => {
                            if app.y_pos > 0 {
                                app.y_pos -= 1;
                                app.path_scroll_amount = 0;
                            }
                        },
                        KeyEvent::Down => {
                            app.y_pos += 1;
                            app.path_scroll_amount = 0;
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
            terminal.clear(ClearType::All)?;
            cursor.goto(0, 0)?;
            for (i, repository) in app.repositories.repositories_sorted()?.iter().enumerate() {
                let size = repository.size();
                if size > 0 {
                    let size_str = match NumberPrefix::binary(size as f64) {
                        Standalone(bytes) => format!("{}", bytes),
                        Prefixed(prefix, n) => format!("{:>5.1} {}B", n, prefix),
                    };
                    let path_str = if i == app.y_pos {
                        scroll_line_if_needed(repository.path().to_string_lossy().to_string(), width as usize - 10, app.path_scroll_amount)
                    } else {
                        scroll_line_if_needed(repository.path().to_string_lossy().to_string(), width as usize - 10, 0)
                    };
                    if i == app.y_pos {
                        terminal.write(Attribute::Reverse)?;
                        terminal.write(format!("{:<10}{}\r\n", size_str, path_str))?;
                        terminal.write(Attribute::Reset)?;
                    } else {
                        terminal.write(format!("{:<10}{}\r\n", size_str, path_str))?;
                    }
                }
                if i >= height as usize - 2 {
                    break;
                }
            }
            if done {
                terminal.write(format!("Done."))?;
            } else {
                terminal.write(format!("{} Searching under {}", spinner_strs[app.spinner_phase], app.root_path))?;
            }
        }
        thread::sleep(Duration::from_millis(33));
        if app.path_scroll_amount < 1000 {
            app.path_scroll_amount += 1;
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
