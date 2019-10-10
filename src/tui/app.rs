use crossterm::{InputEvent, KeyEvent};

use crate::repository::RepositoryStore;
use crate::tui::{pathlist, pathlist::PathList};
use crate::tui::usagebar::UsageBar;
use crate::tui::statusbar::StatusBar;
use crate::tui::helpwindow::HelpWindow;
use crate::tui::{details, details::Details};

pub enum AppState {
    PathList,
    Details,
}

pub struct App {
    pub repositories: RepositoryStore,
    pub root_path: String,
    pub path_list: PathList,
    pub usage_bar: UsageBar,
    pub status_bar: StatusBar,
    pub details: Details,
    pub help_window: HelpWindow,
    pub state: AppState,
}

impl App {
    pub fn input(&mut self, event: InputEvent) -> Result<bool, Box<dyn std::error::Error>> {
        match event.clone() {
            InputEvent::Keyboard(k) => {
                match k {
                    KeyEvent::Char(c) => match c {
                        'q' => {
                            return Ok(true);
                        },
                        'h' => {
                            self.help_window.show = !self.help_window.show;
                        },
                        _ => {},
                    },
                    KeyEvent::Ctrl('c') => {
                        return Ok(true);
                    },
                    _ => {},
                }
            }
            _ => {},
        }
        if !self.help_window.show {
            match self.state {
                AppState::PathList => {
                    match self.path_list.input(event.clone(), &self.repositories)? {
                        Some(pathlist::Event::Open(repository)) => {
                            self.details.repository = Some(repository);
                            self.state = AppState::Details;
                        },
                        None => {},
                    }
                },
                AppState::Details => {
                    match self.details.input(event.clone())? {
                        Some(details::Event::Close) => {
                            self.details.repository = None;
                            self.state = AppState::PathList;
                        },
                        None => {},
                    }
                }
            }
        }
        Ok(false)
    }

    pub fn draw(&self) -> Result<(), Box<dyn std::error::Error>> {
        let cursor = crossterm::cursor();
        cursor.goto(0, 0)?;
        self.usage_bar.draw()?;
        let repositories = self.repositories.repositories_sorted()?;
        match self.state {
            AppState::PathList => {
                self.path_list.draw(&repositories)?;
            },
            AppState::Details => {
                self.details.draw()?;
            }
        }
        self.status_bar.draw(&self.root_path)?;
        if self.help_window.show {
            self.help_window.draw()?;
        }
        Ok(())
    }
}

