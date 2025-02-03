pub enum Symbol {
    BoxHorizontal,
    BoxVertical,
    BoxTopLeft,
    BoxTopRight,
    BoxBottomLeft,
    BoxBottomRight,

    BoxDoubleHorizontal,
    BoxDoubleVertical,
    BoxDoubleTopLeft,
    BoxDoubleTopRight,
    BoxDoubleBottomLeft,
    BoxDoubleBottomRight,

    BoxHeavyFull,
    BoxHeavyUp,
    BoxHeavyDown,
}

impl Symbol {
    pub const fn as_str(&self) -> &'static str {
        match self {
            Symbol::BoxHorizontal => "─",
            Symbol::BoxVertical => "│",
            Symbol::BoxTopLeft => "┌",
            Symbol::BoxTopRight => "┐",
            Symbol::BoxBottomLeft => "└",
            Symbol::BoxBottomRight => "┘",

            Symbol::BoxDoubleHorizontal => "═",
            Symbol::BoxDoubleVertical => "║",
            Symbol::BoxDoubleTopLeft => "╔",
            Symbol::BoxDoubleTopRight => "╗",
            Symbol::BoxDoubleBottomLeft => "╚",
            Symbol::BoxDoubleBottomRight => "╝",

            Symbol::BoxHeavyFull => "█",
            Symbol::BoxHeavyUp => "▀",
            Symbol::BoxHeavyDown => "▄",
        }
    }
}

use std::fmt;

impl fmt::Display for Symbol {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let symbol = self.as_str();

        write!(f, "{}", symbol)
    }
}
