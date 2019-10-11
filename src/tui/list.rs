use crossterm::{InputEvent, KeyEvent, ClearType, Attribute};
use std::cmp::min;

pub struct List {
    pub pos: usize,
    pub offset: usize,
    pub height: usize,
}

impl List {
    pub fn go_up(&mut self) {
        if self.pos > 0 {
            self.pos -= 1;
        } else if self.offset > 0 {
            self.offset -= 1;
        }
    }

    pub fn go_down(&mut self, list_len: usize) {
        if self.pos + 1 < min(self.height, list_len) {
            self.pos += 1;
        } else if self.offset + self.pos + 1 < list_len {
            self.offset += 1;
        }
    }

    pub fn go_to_top(&mut self) {
        self.pos = 0;
        self.offset = 0;
    }

    pub fn go_to_bottom(&mut self, list_len: usize) {
        if list_len < self.height {
            self.pos = list_len - 1;
        } else {
            self.pos = self.height - 1;
            self.offset = list_len - self.height;
        }
    }

    pub fn draw(&self, strs: &Vec<String>) -> crossterm::Result<()> {
        let terminal = crossterm::terminal();
        for i in 0..self.height {
            if let Some(string) = strs.get(self.offset + i) {
                terminal.clear(ClearType::CurrentLine)?;
                if i == self.pos {
                    terminal.write(Attribute::Reverse)?;
                    terminal.write(string)?;
                    terminal.write(Attribute::Reset)?;
                } else {
                    terminal.write(string)?;
                }
            } else {
                terminal.clear(ClearType::CurrentLine)?;
                terminal.write("\r\n")?;
            }
        }
        Ok(())
    }

    pub fn input(&mut self, event: InputEvent, list_len: usize) {
        match event {
            InputEvent::Keyboard(k) => {
                match k {
                    KeyEvent::Char(c) => match c {
                        'j' => {
                            self.go_down(list_len);
                        },
                        'k' => {
                            self.go_up();
                        },
                        'g' => {
                            self.go_to_top();
                        },
                        'G' => {
                            self.go_to_bottom(list_len);
                        },
                        _ => {},
                    },
                    KeyEvent::Up => {
                        self.go_up();
                    },
                    KeyEvent::Down => {
                        self.go_down(list_len);
                    }
                    _ => {},
                }
            }
            _ => {},
        }
    }
}
