use crate::definitions::{Size, Position};
use crate::font::Font;
use crate::rich_string::RichString;


#[derive(Copy, Clone)]
pub enum Rich {
    Font(Font),
    Reset,
}

#[derive(Copy, Clone)]
pub struct RichChar {
    pub data: char,
    pub font: Option<Rich>,
}

pub struct Canvas {
    pub size: Size,
    pub data: Vec<Vec<RichChar>>,
}

impl Canvas {
    pub fn new(size: Size) -> Self {
        Self {
            size,
            data: vec![vec![RichChar{data: '\0', font: None}; size.width]; size.height],
        }
    }

    pub fn clear(&mut self) {
        self.data.fill(vec![RichChar{data: '\0', font: None}; self.size.width]);
    }

    pub fn add_string(&mut self, string: &str, position: Position) {
        let mut position_x: usize = position.x;
        let mut position_y: usize = position.y;

        for character in string.chars() {
            if character == '\n' {
                position_x = position.x;
                position_y += 1;
            } else {
                self.data[position_y][position_x].data = character;
                position_x += 1;
            }
        }
    }

    pub fn add_rich_string(&mut self, string: &RichString, position: Position) {
        let mut position_x: usize = position.x;
        let mut position_y: usize = position.y;

        let mut new_line: bool = true;

        for character in string.get_text().chars() {
            if new_line {
                self.data[position_y][position_x].font = Some(Rich::Font(string.get_font()));
            }

            if character == '\n' {
                self.data[position_y][position_x].font = Some(Rich::Reset);

                new_line = true;

                position_x = position.x;
                position_y += 1;
            } else {
                new_line = false;

                self.data[position_y][position_x].data = character;
                position_x += 1;
            }
        }

        self.data[position_y][position_x].font = Some(Rich::Reset);
    }
}
