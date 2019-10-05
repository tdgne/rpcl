use crate::repository::RepositoryStore;
use crate::tui::pathlist::PathList;
use crate::tui::usagebar::UsageBar;
use crate::tui::statusbar::StatusBar;

pub struct App {
    pub repositories: RepositoryStore,
    pub root_path: String,
    pub path_list: PathList,
    pub usage_bar: UsageBar,
    pub status_bar: StatusBar,
    
}

impl App {
    pub fn draw(&self) -> Result<(), Box<dyn std::error::Error>> {
        let cursor = crossterm::cursor();
        cursor.goto(0, 0)?;
        self.usage_bar.draw()?;
        let repositories = self.repositories.repositories_sorted()?;
        self.path_list.draw(&repositories)?;
        self.status_bar.draw(&self.root_path)?;
        Ok(())
    }
}

