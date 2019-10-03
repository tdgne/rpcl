use std::error::Error;
use std::thread;
use std::time::Duration;
use std::sync::mpsc::{Receiver, channel};
use clap;
use crossterm::{style, Attribute, RawScreen, input, InputEvent, KeyEvent, AlternateScreen, ClearType, Color, Crossterm};
use number_prefix::{NumberPrefix, Standalone, Prefixed};

pub mod repository;
use repository::{Repository, RepositoryStore};

pub mod collector;
use collector::*;

fn main() -> Result<(), Box<dyn Error>> {
    let matches = clap::App::new("Repository Locator")
        .version("0.1.0")
        .author("tdgne")
        .about("Locates repositories")
        .arg(clap::Arg::with_name("DIR")
             .help("Sets the root directory to start searching")
             .index(1))
        .get_matches();
    let root_path = matches.value_of("DIR").unwrap_or(".").to_owned();
    let (tx, rx) = channel();
    let repositories = RepositoryStore::with_sender(tx);
    {
        let repositories = repositories.clone();
        let collector = thread::spawn(|| {
            collect_repositories(root_path, repositories).unwrap();
        });
    }

    let _alt = AlternateScreen::to_alternate(false)?;
    let _raw = RawScreen::into_raw_mode()?;
    let crossterm = Crossterm::new();
    let terminal = crossterm.terminal();
    let cursor = crossterm.cursor();

    let input = input();
    let mut stdin = input.read_async();

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
        if let Ok(event) = rx.try_recv() {
            match event {
                repository::Event::Update => {
                    render = true;
                }
            }
        }
        if render {
            terminal.clear(ClearType::All)?;
            cursor.goto(0, 0)?;
            for (i, repository) in repositories.repositories_sorted()?.iter().enumerate() {
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
        }
        thread::sleep(Duration::from_millis(33));
    }

    Ok(())
}
