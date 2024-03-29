use crate::tui::Window;

pub struct HelpWindow {
    pub show: bool,
    pub window: Window,
}

const MESSAGE: &'static [&'static str] = &[
    "  j,k: Move up, down",
    "  g,G: Go to top, bottom",
    "Enter: Show details of the selected repository",
    "    d: Delete selected resources",
    "    h: Show this message",
];

impl HelpWindow {
    pub fn new() -> Self {
        Self {
            show: false,
            window: Window {
                message: MESSAGE.iter().map(|m| m.to_string()).collect::<Vec<_>>(),
            }
        }
    }

    pub fn draw(&self) -> Result<(), Box<dyn std::error::Error>> {
        self.window.draw()?;
        Ok(())
    }
}
