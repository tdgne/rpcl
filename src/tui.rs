use std::error::Error;
use std::thread;
use std::time::Duration;
use std::sync::mpsc::{channel, Receiver};
use crossterm::{style, Attribute, Terminal, RawScreen, input, InputEvent, KeyEvent, AlternateScreen, ClearType, Color, Crossterm, Styler};

use crate::collector;
use crate::repository::*;

mod app;
pub use app::App;

mod pathlist;
pub use pathlist::PathList;

mod usagebar;
pub use usagebar::UsageBar;

mod statusbar;
pub use statusbar::StatusBar;

pub fn run_tui(
    repositories: RepositoryStore,
    root_path: String,
    collector_rx: Receiver<collector::Event>,
) -> Result<(), Box<dyn Error>> {
    let (spinner_tx, spinner_rx) = channel();
    let _spinner = thread::spawn(move || {
        loop {
            thread::sleep(Duration::from_millis(333));
            spinner_tx.send(()).unwrap();
        }
    });

    let _raw = RawScreen::into_raw_mode()?;
    let crossterm = Crossterm::new();
    let terminal = crossterm.terminal();
    terminal.clear(ClearType::All)?;
    let cursor = crossterm.cursor();
    cursor.hide()?;
    let (width, height) = terminal.size()?;

    let mut app = App {
        repositories: repositories.clone(),
        root_path,
        path_list: PathList {
            pos: 0,
            offset: 0,
            path_scroll_amount: 0,
            height: height as usize - 2
        },
        usage_bar: UsageBar,
        status_bar: StatusBar {
            spinner_phase: 0,
            done: false,
        }
    };

    let input = input();
    let mut stdin = input.read_async();

    let spinner_strs = ["◡◡", "⊙⊙", "◠◠", "⊙⊙"];

    loop {
        let (width, height) = terminal.size()?;
        app.path_list.height = height as usize - 2;


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
                                app.path_list.go_down(repositories.filtered_len()?);
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
                            app.path_list.go_down(repositories.filtered_len()?);
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
                    app.status_bar.done = true;
                    render = true;
                }
            }
        }
        if let Ok(_) = spinner_rx.try_recv() {
            render = true;
            app.status_bar.spinner_phase += 1;
            app.status_bar.spinner_phase %= 4;
        }
        if render {
            app.draw()?;
        }
        thread::sleep(Duration::from_millis(67));
        if app.path_list.path_scroll_amount < 1000 {
            app.path_list.path_scroll_amount += 1;
        }
    }

    cursor.show()?;

    Ok(())
}
