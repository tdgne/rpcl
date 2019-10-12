use crossterm::ClearType;

use crate::tui::app::AppState;

pub struct UsageBar;

impl UsageBar {
    pub fn draw(&self, state: &AppState) -> crossterm::Result<()> {
        let terminal = crossterm::terminal();
        terminal.clear(ClearType::CurrentLine)?;
        match state {
            &AppState::PathList => {
                terminal.write("j,k: Move | Enter: Details | q: Quit | h: Help\r\n")?;
            },
            &AppState::Details(_) => {
                terminal.write("j,k: Move | Enter: Back to list | d: Delete | q: Quit | h: Help\r\n")?;
            },
        }
        Ok(())
    }
}
