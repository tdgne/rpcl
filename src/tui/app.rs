use std::path::{Path, PathBuf};

use crossterm::{InputEvent, KeyEvent};

use crate::repository::{Repository, RepositoryStore, IgnoredPathInfo};
use crate::tui::{pathlist, pathlist::PathList};
use crate::tui::usagebar::UsageBar;
use crate::tui::statusbar::StatusBar;
use crate::tui::helpwindow::HelpWindow;
use crate::tui::{details, details::Details};

#[derive(Clone)]
pub enum AppState {
    PathList,
    Details(PathBuf),
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
                            self.state = AppState::Details(repository.path().to_path_buf());
                        },
                        None => {},
                    }
                },
                AppState::Details(ref path) => {
                    let repository = self.repositories.find_by_path(path.clone())?.expect("Repository not found");
                    match self.details.input(event.clone(), &repository)? {
                        Some(details::Event::Close) => {
                            self.state = AppState::PathList;
                        },
                        Some(details::Event::DeleteAll) => {
                            self.clean_repository(&repository)?;
                        },
                        Some(details::Event::Delete(path)) => {
                            self.clean_ignored_path(
                                repository.clone(),
                                repository.ignored_path_infos()
                                    .iter()
                                    .find(|i| i.path() == path.as_path())
                                    .expect("IgnoredPathInfo not found")
                                    .clone())?;
                        },
                        None => {},
                    }
                },
            }
        }
        Ok(false)
    }

    pub fn clean_repository(&mut self, repository: &Repository) -> Result<(), Box<dyn std::error::Error>> {
        for info in repository.ignored_path_infos().iter() {
            self.clean_ignored_path(repository.clone(), info.clone())?;
        }
        Ok(())
    }

    pub fn clean_ignored_path(&mut self, repository: Repository, ignored_path_info: IgnoredPathInfo) -> Result<(), Box<dyn std::error::Error>> {
        let mut repositories = self.repositories.clone();
        std::thread::spawn(move || {
            repositories.clean_ignored_path(&repository, &ignored_path_info);
        });
        Ok(())
    }

    pub fn draw(&self) -> Result<(), Box<dyn std::error::Error>> {
        let cursor = crossterm::cursor();
        cursor.goto(0, 0)?;
        self.usage_bar.draw(&self.state)?;
        let repositories = self.repositories.repositories_sorted()?;
        match self.state {
            AppState::PathList => {
                self.path_list.draw(&repositories)?;
            },
            AppState::Details(ref path) => {
                let repository = self.repositories.find_by_path(path.clone())?;
                if let Some(repository) = repository {
                    self.details.draw(repository)?;
                }
            }
        }
        self.status_bar.draw(&self.root_path)?;
        if self.help_window.show {
            self.help_window.draw()?;
        }
        Ok(())
    }
}

