use crate::definitions::Position;
use crate::canvas::{Canvas, DEFAULT_FONT};
use crate::font::{ColorDecoration, Reset, ResetDecoration};

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
        let mut font_last = DEFAULT_FONT;

        Self::screen_clear();

        print!("{}", font_last);

        for position_y in 0..self.size.height {
            for position_x in 0..self.size.width {
                if self.data[position_y][position_x].font.as_str() != font_last.as_str() {
                    for i in 0..3 {
                        if let Some(decoration) = font_last.get_decoration()[i] {
                            match decoration {
                                ColorDecoration::Bold => print!("{}", ResetDecoration.bold_as_str()),
                                ColorDecoration::Underline => print!("{}", ResetDecoration.underline_as_str()),
                                ColorDecoration::Reversed => print!("{}", ResetDecoration.reversed_as_str()),
                            }
                        }
                    }

                    font_last = self.data[position_y][position_x].font;
                    print!("{}", font_last);
                }
                print!("{}", self.data[position_y][position_x].data);
            }
            if position_y + 1 < self.size.height { println!(""); }
        }
        print!("{}", Reset);
        Self::screen_flush();
    }
}
