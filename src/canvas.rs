use crate::definitions::{Size, Position};
use crate::font::{Color, Font};
use crate::rich_string::RichString;

pub const DEFAULT_COLOR_BACKGROUND: Color = Color::Black;
pub const DEFAULT_COLOR_TEXT: Color = Color::White;
pub const DEFAULT_FONT: Font = Font::new(DEFAULT_COLOR_BACKGROUND, DEFAULT_COLOR_TEXT);

#[derive(Copy, Clone)]
pub struct CanvasChar {
    pub data: char,
    pub font: Font,
}

pub struct Canvas {
    pub size: Size,
    pub data: Vec<Vec<CanvasChar>>,
}

impl Canvas {
    pub fn new(size: Size) -> Self {
        Self {
            size,
            data: vec![vec![CanvasChar{data: ' ', font: DEFAULT_FONT}; size.width]; size.height],
        }
    }

    pub fn clear(&mut self) {
        self.data.fill(vec![CanvasChar{data: ' ', font: DEFAULT_FONT}; self.size.width]);
    }

    pub fn add_string(&mut self, string: &RichString, position: Position) {
        let mut array_position = position;

        for character in string.get_text().chars() {
            if character == '\n' {
                array_position.y += 1;
                array_position.x = position.x;
            } else {
                self.data[array_position.y][array_position.x].font = string.get_font();
                self.data[array_position.y][array_position.x].data = character;
                array_position.x += 1;
            }
        }
    }

}
