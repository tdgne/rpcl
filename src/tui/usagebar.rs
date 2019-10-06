use crossterm::ClearType;

pub struct UsageBar;

impl UsageBar {
    pub fn draw(&self) -> crossterm::Result<()> {
        let terminal = crossterm::terminal();
        terminal.clear(ClearType::CurrentLine)?;
        terminal.write("q: Quit  j,k: Move\r\n")?;
        Ok(())
    }
}