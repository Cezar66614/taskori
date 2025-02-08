use crate::definitions::Position;
use crate::canvas::*;
use crate::font::*;

use std::io::{self, Write};

pub type Screen = Canvas;

impl Screen {
    fn screen_flush() { io::stdout().flush().unwrap(); }
    fn screen_clear() { print!("\x1B[2J\x1B[H"); Self::screen_flush(); }

    pub fn add_canvas(&mut self, canvas: &Canvas, position: Position) {
        let mut canvas_pos: Position = Position{x: 0, y: 0};

        for _y in canvas.data.iter() {
            for _x in _y.iter() {
                self.data[position.y + canvas_pos.y][position.x + canvas_pos.x] = canvas.data[canvas_pos.y][canvas_pos.x];

                canvas_pos.x += 1;
            }

            canvas_pos.y += 1;
            canvas_pos.x = 0;
        }
    }

    pub fn print(&self) {
        Self::screen_clear();
        for position_y in 0..self.size.height {
            for position_x in 0..self.size.width {
                if let Some(font) = self.data[position_y][position_x].font {
                    print!("{}", match font {
                        FontTypes::Font(font) => font.as_string(),
                        _ => String::new(),
                    })
                }
                print!("{}", self.data[position_y][position_x].data);
                if let Some(font) = self.data[position_y][position_x].font {
                    print!("{}", match font {
                        FontTypes::Reset => Reset.as_str(),
                        _ => "",
                    })
                }
            }
            if position_y + 1 < self.size.height { println!(""); }
        }
        Self::screen_flush();
    }
}
