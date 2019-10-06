use strfmt::strfmt;
use std::collections::HashMap;

pub struct Window {
    pub message: Vec<String>,
}

impl Window {
    pub fn draw(&self) -> Result<(), Box<dyn std::error::Error>> {
        let message_width = self.message.iter().map(|m| m.len() as u16).max().expect("Invalid message");
        let message_height = self.message.len() as u16;
        let terminal = crossterm::terminal();
        let cursor = crossterm::cursor();
        let (terminal_width, terminal_height) = terminal.size()?;
        if terminal_width < message_width || terminal_height < message_height {
            // return an Error
            return Ok(());
        }
        let x = terminal_width / 2 - message_width / 2;
        let y = terminal_height / 2 - message_height / 2;
        let padding = 2;
        if x < padding || y < padding {
            // return an Error
            return Ok(());
        }
        for y in (y - padding)..y {
            cursor.goto(x - padding, y)?;
            for _ in 0..(message_width + padding * 2) {
                terminal.write(" ")?;
            }
        }
        for (i, y) in (y..(y + message_height)).enumerate() {
            cursor.goto(x - padding, y)?;
            let mut map = HashMap::new();
            map.insert("message".to_string(), &self.message[i]);
            for _ in 0..padding {
                terminal.write(" ")?;
            }
            terminal.write(strfmt(&format!("{{message:<{}}}", message_width), &map)?)?;
            for _ in 0..padding {
                terminal.write(" ")?;
            }
        }
        for y in (y + message_height)..(y + message_height + padding) {
            cursor.goto(x - padding, y)?;
            for _ in 0..(message_width + padding * 2) {
                terminal.write(" ")?;
            }
        }
        Ok(())
    }
}
