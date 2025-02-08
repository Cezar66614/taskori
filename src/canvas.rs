use crate::definitions::{Size, Position};
use crate::font::{FontTypes, Reset};
use crate::rich_string::RichString;

#[derive(Copy, Clone)]
pub struct CanvasChar {
    pub data: char,
    pub font: Option<FontTypes>,
}

pub struct Canvas {
    pub size: Size,
    pub data: Vec<Vec<CanvasChar>>,
}

impl Canvas {
    pub fn new(size: Size) -> Self {
        Self {
            size,
            data: vec![vec![CanvasChar{data: ' ', font: None}; size.width]; size.height],
        }
    }

    pub fn clear(&mut self) {
        self.data.fill(vec![CanvasChar{data: ' ', font: None}; self.size.width]);
    }

    pub fn add_string(&mut self, string: &str, position: Position) {
        let mut array_position = position;

        for character in string.chars() {
            if character == '\n' {
                array_position.y += 1;
                array_position.x = position.x;
            } else {
                self.data[array_position.y][array_position.x].data = character;
                array_position.x += 1;
            }
        }
    }

    pub fn add_rich_string(&mut self, string: &RichString, position: Position) {
        let mut array_position = position;
        let mut last_position = array_position;

        let mut active = false;

        for character in string.get_text().chars() {
            if character == '\n' {
                if active {
                    self.data[last_position.y][last_position.x].font = Some(FontTypes::Reset);
                    active = false;
                }

                array_position.y += 1;
                array_position.x = position.x;
            } else {
                if !active {
                    self.data[array_position.y][array_position.x].font = Some(FontTypes::Font(string.get_font()));
                    active = true;
                }

                self.data[array_position.y][array_position.x].data = character;

                last_position = array_position;
                array_position.x += 1;
            }
        }

        if active {
            self.data[last_position.y][last_position.x].font = Some(FontTypes::Reset);
        }
    }

    pub fn as_str(&self) -> String {
        let mut out = String::new();
        let mut position = Position{x: 0, y: 0};

        for _y in self.data.iter() {
            for _x in _y.iter() {
                if let Some(font) = self.data[position.y][position.x].font {
                    let data: String;
                    data = match font {
                        FontTypes::Font(font) => font.as_string(),
                        _ => String::new(),
                    };
                    out += &data;
                }
                out += &self.data[position.y][position.x].data.to_string();
                if let Some(font) = self.data[position.y][position.x].font {
                    out += match font {
                        FontTypes::Reset => Reset.as_str(),
                        _ => "",
                    }
                }

                position.x += 1;
            }

            if position.y + 1 < self.size.height { out += "\n"; }

            position.y += 1;
            position.x = 0;
        }

        out
    }
}
