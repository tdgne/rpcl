use std::error::Error;
use std::thread;
use std::time::Duration;
use std::sync::mpsc::{channel, Receiver};
use crossterm::{RawScreen, input, InputEvent, KeyEvent, ClearType, Crossterm};

use crate::collector;
use crate::repository::*;

mod app;
pub use app::{App, AppState};

mod list;
pub use list::List;

mod pathlist;
pub use pathlist::PathList;

mod usagebar;
pub use usagebar::UsageBar;

mod statusbar;
pub use statusbar::StatusBar;

mod details;
pub use details::Details;

mod helpwindow;
pub use helpwindow::HelpWindow;

mod window;
pub use window::Window;

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
    let (_width, height) = terminal.size()?;

    let mut app = App {
        repositories: repositories.clone(),
        root_path,
        path_list: PathList {
            list: List {
                pos: 0,
                offset: 0,
                height: height as usize - 2
            },
            path_scroll_amount: 0,
        },
        usage_bar: UsageBar,
        status_bar: StatusBar {
            spinner_phase: 0,
            done: false,
        },
        details: Details {
            list: List {
                pos: 0,
                offset: 0,
                height: height as usize - 2,
            }
        },
        help_window: HelpWindow::new(),
        state: AppState::PathList,
    };

    let input = input();
    let mut stdin = input.read_async();

    loop {
        let (_width, height) = terminal.size()?;
        app.path_list.list.height = height as usize - 2;
        app.details.list.height = height as usize - 2;

        if let Some(event) = stdin.next() {
            if app.input(event)? {
                break;
            }
        }
        if let Ok(event) = collector_rx.try_recv() {
            match event {
                collector::Event::Update => {
                },
                collector::Event::Done => {
                    app.status_bar.done = true;
                }
            }
        }
        if let Ok(_) = spinner_rx.try_recv() {
            app.status_bar.spinner_phase += 1;
            app.status_bar.spinner_phase %= 4;
        }

        app.draw()?;

        thread::sleep(Duration::from_millis(67));
        if app.path_list.path_scroll_amount < 1000 {
            app.path_list.path_scroll_amount += 1;
        }
    }

    cursor.show()?;

    Ok(())
}
