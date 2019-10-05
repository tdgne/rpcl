use crossterm::ClearType;

const SPINNER_STRS: &'static [&'static str] = &["◡◡", "⊙⊙", "◠◠", "⊙⊙"];

pub struct StatusBar {
    pub done: bool,
    pub spinner_phase: usize,
}

impl StatusBar {
    pub fn draw(&self, root_path: &String) -> crossterm::Result<()> {
        let terminal = crossterm::terminal();
        terminal.clear(ClearType::CurrentLine)?;
        if self.done {
            terminal.write(format!("Done."))?;
        } else {
            terminal.write(format!("{} Searching under {}", SPINNER_STRS[self.spinner_phase], root_path))?;
        }
        Ok(())
    }
}
