enum ColorType {
    Text,
    Background,
}

#[derive(Copy, Clone)]
pub enum ColorDecoration {
    Bold,
    Underline,
    Reversed,
}

#[derive(Copy, Clone)]
pub enum Color {
    Black,
    Red,
    Green,
    Yellow,
    Blue,
    Magenta,
    Cyan,
    White,
}

pub struct Reset;

impl Reset {
    const RESET_CODE: &'static str = "\u{001b}[0m";

    pub const fn as_str(&self) -> &'static str {
        Self::RESET_CODE
    }
}

#[derive(Copy, Clone)]
pub struct Font {
    background_color: Color,
    text_color: Color,
    decorations: [Option<ColorDecoration>;3],
}

impl Font {
    pub fn new(background_color: Color, text_color: Color) -> Self {
        Self {
            background_color,
            text_color,
            decorations: [None;3],
        }
    }

    /// ANSI escape codes table (Indexed by base color index)
    /// - `index = base_color - 30`

    const TEXT_COLOR_TABLE: [&str; 8] = [
        "\u{001b}[30m", // Black
        "\u{001b}[31m", // Red
        "\u{001b}[32m", // Green
        "\u{001b}[33m", // Yellow
        "\u{001b}[34m", // Blue
        "\u{001b}[35m", // Magenta
        "\u{001b}[36m", // Cyan
        "\u{001b}[37m", // White
    ];
    const BACKGROUND_COLOR_TABLE: [&str; 8] = [
        "\u{001b}[40m", // Black
        "\u{001b}[41m", // Red
        "\u{001b}[42m", // Green
        "\u{001b}[43m", // Yellow
        "\u{001b}[44m", // Blue
        "\u{001b}[45m", // Magenta
        "\u{001b}[46m", // Cyan
        "\u{001b}[47m", // White
    ];

    const DECORATION: [&str; 3] = [
        "\u{001b}[1m", // Bold
        "\u{001b}[4m", // Underline
        "\u{001b}[7m", // Reversed
    ];

    /// Table lookup function
    const fn lookup(index: usize, r#type: ColorType) -> &'static str {
        match r#type {
            ColorType::Text => Self::TEXT_COLOR_TABLE[index],
            ColorType::Background => Self::BACKGROUND_COLOR_TABLE[index],
        }
    }
    const fn lookup_decoration(decoration: &[Option<ColorDecoration>;3]) -> [Option<&'static str>;3] {
        let mut out: [Option<&'static str>;3] = [None, None, None];

        if let Some(_) = decoration[0] {
            out[0] = Some(Self::DECORATION[0]);
        }
        if let Some(_) = decoration[1] {
            out[1] = Some(Self::DECORATION[1]);
        }
        if let Some(_) = decoration[2] {
            out[2] = Some(Self::DECORATION[2]);
        }

        out
    }

    pub const fn as_str(&self) -> (&'static str, &'static str, [Option<&'static str>;3]) {
        /*
        let background = match self.background_color {
            Color::Black   => Self::lookup(0, ColorType::Background),
            Color::Red     => Self::lookup(1, ColorType::Background),
            Color::Green   => Self::lookup(2, ColorType::Background),
            Color::Yellow  => Self::lookup(3, ColorType::Background),
            Color::Blue    => Self::lookup(4, ColorType::Background),
            Color::Magenta => Self::lookup(5, ColorType::Background),
            Color::Cyan    => Self::lookup(6, ColorType::Background),
            Color::White   => Self::lookup(7, ColorType::Background),
        };
        */
        let background = Self::lookup(self.background_color as usize, ColorType::Background);

        /*
        let text = match self.text_color {
            Color::Black   => Self::lookup(0, ColorType::Text),
            Color::Red     => Self::lookup(1, ColorType::Text),
            Color::Green   => Self::lookup(2, ColorType::Text),
            Color::Yellow  => Self::lookup(3, ColorType::Text),
            Color::Blue    => Self::lookup(4, ColorType::Text),
            Color::Magenta => Self::lookup(5, ColorType::Text),
            Color::Cyan    => Self::lookup(6, ColorType::Text),
            Color::White   => Self::lookup(7, ColorType::Text),
        };
        */
        let text = Self::lookup(self.text_color as usize, ColorType::Text);

        (background, text, Self::lookup_decoration(&self.decorations))
    }

    pub fn as_string(&self) -> String {
        let mut out = String::new();

        let font = self.as_str();

        out += font.0;
        out += font.1;

        for i in 0..3 {
            if let Some(decoration) = font.2[i] {
                out += decoration;
            }
        }

        out
    }

    pub fn decoration_set(&mut self, decoration: ColorDecoration) {
        match decoration {
            ColorDecoration::Bold      => self.decorations[0] = Some(ColorDecoration::Bold),
            ColorDecoration::Underline => self.decorations[1] = Some(ColorDecoration::Underline),
            ColorDecoration::Reversed  => self.decorations[2] = Some(ColorDecoration::Reversed),
        }
    }
    pub fn decoration_unset(&mut self, decoration: ColorDecoration) {
        match decoration {
            ColorDecoration::Bold      => self.decorations[0] = None,
            ColorDecoration::Underline => self.decorations[1] = None,
            ColorDecoration::Reversed  => self.decorations[2] = None,
        }
    }
    pub fn decoration_reset(&mut self) { self.decorations = [None;3]; }

    pub fn color_set_background(&mut self, color: Color) { self.background_color = color; }
    pub fn color_set_text(&mut self, color: Color) { self.text_color = color; }

}

#[derive(Copy, Clone)]
pub enum FontTypes {
    Font(Font),
    Reset,
}

use std::fmt;

impl fmt::Display for Reset {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

impl fmt::Display for Font {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let (background, text, decoration) = self.as_str();

        for i in 0..3 {
            if let Some(decoration) = decoration[i] {
                write!(f, "{}", decoration)?
            }
        }

        write!(f, "{}{}", background, text)
    }
}
