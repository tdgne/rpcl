use std::error::Error;
use std::thread;
use std::time::Duration;
use std::sync::mpsc::Receiver;
use crossterm::{style, Attribute, RawScreen, input, InputEvent, KeyEvent, AlternateScreen, ClearType, Color, Crossterm};
use number_prefix::{NumberPrefix, Standalone, Prefixed};

use crate::collector;
use crate::repository::*;

pub struct App {
    pub repositories: RepositoryStore,
    pub root_path: String,
    pub spinner_phase: usize,
}

pub fn run_tui(
    mut app: App,
    collector_rx: Receiver<collector::Event>,
    spinner_rx: Receiver<()>
) -> Result<(), Box<dyn Error>> {
    let _alt = AlternateScreen::to_alternate(false)?;
    let _raw = RawScreen::into_raw_mode()?;
    let crossterm = Crossterm::new();
    let terminal = crossterm.terminal();
    let cursor = crossterm.cursor();

    let input = input();
    let mut stdin = input.read_async();

    let spinner_strs = ["◡◡", "⊙⊙", "◠◠", "⊙⊙"];
    let mut done = false;

    loop {
        let mut render = false;
        if let Some(event) = stdin.next() {
            match event {
                InputEvent::Keyboard(k) => {
                    match k {
                        KeyEvent::Char(c) => match c {
                            'q' => {
                                break;
                            },
                            _ => {},
                        },
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
            cursor.hide()?;
            for (i, repository) in app.repositories.repositories_sorted()?.iter().enumerate() {
                let size = repository.size();
                if size > 0 {
                    let size_str = match NumberPrefix::binary(size as f64) {
                        Standalone(bytes) => format!("{}", bytes),
                        Prefixed(prefix, n) => format!("{:>5.1} {}B", n, prefix),
                    };
                    terminal.write(format!("{} {}\r\n", size_str, repository.path().to_string_lossy()))?;
                }
                if i > 20 {
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
    }

    Ok(())
}
