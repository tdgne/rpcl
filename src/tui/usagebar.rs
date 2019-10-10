use crossterm::ClearType;

pub struct UsageBar;

impl UsageBar {
    pub fn draw(&self) -> crossterm::Result<()> {
        let terminal = crossterm::terminal();
        terminal.clear(ClearType::CurrentLine)?;
        terminal.write("j,k: Move  Enter: Details  q: Quit  h: Help\r\n")?;
        Ok(())
    }
}
