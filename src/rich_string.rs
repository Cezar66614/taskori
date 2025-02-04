use crate::font::*;

pub struct RichString {
    text: String,
    font: Font,
}

impl RichString {
    pub fn new(text: &str, font: Font) -> Self {
        Self {
            text: String::from(text),
            font,
        }
    }

    pub fn get_text(&self) -> &str {
        &self.text
    }
    pub fn get_font(&self) -> Font {
        self.font
    }

    pub fn as_str(&self) -> String {
        let mut out = String::new();
        
        out += &self.font.as_string();

        out += &self.text;

        out += Reset.as_str();

        out
    }
}

use std::fmt;

impl fmt::Display for RichString {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
}
