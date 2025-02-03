use crate::symbol::*;

pub struct Size { pub width: u32, pub height: u32 }

pub enum ContainerStyle {
    Single,
    Double,
    Heavy,
}

pub struct Container {
    size: Size,
    style: ContainerStyle,
}

impl Container {
    pub fn new(size: Size, style: ContainerStyle) -> Self {
        Self {
            size,
            style,
        }
    }

    pub fn as_str(&self) -> String {
        let mut r#box = String::new();

        match self.style {
            ContainerStyle::Single => {
                // Top
                r#box += Symbol::BoxTopLeft.as_str();

                for _i in 1..=self.size.width {
                    r#box += Symbol::BoxHorizontal.as_str();
                }

                r#box += Symbol::BoxTopRight.as_str();

                r#box += "\n";

                // Middle
                for _j in 1..=self.size.height {
                    r#box += Symbol::BoxVertical.as_str();

                    for _i in 1..=self.size.width {
                        r#box += " ";
                    }

                    r#box += Symbol::BoxVertical.as_str();

                    r#box += "\n";
                }

                // Bottom
                r#box += Symbol::BoxBottomLeft.as_str();

                for _i in 1..=self.size.width {
                    r#box += Symbol::BoxHorizontal.as_str();
                }

                r#box += Symbol::BoxBottomRight.as_str();
            }
            ContainerStyle::Double => {
                // Top
                r#box += Symbol::BoxDoubleTopLeft.as_str();

                for _i in 1..=self.size.width {
                    r#box += Symbol::BoxDoubleHorizontal.as_str();
                }

                r#box += Symbol::BoxDoubleTopRight.as_str();

                r#box += "\n";

                // Middle
                for _j in 1..=self.size.height {
                    r#box += Symbol::BoxDoubleVertical.as_str();

                    for _i in 1..=self.size.width {
                        r#box += " ";
                    }

                    r#box += Symbol::BoxDoubleVertical.as_str();

                    r#box += "\n";
                }

                // Bottom
                r#box += Symbol::BoxDoubleBottomLeft.as_str();

                for _i in 1..=self.size.width {
                    r#box += Symbol::BoxDoubleHorizontal.as_str();
                }

                r#box += Symbol::BoxDoubleBottomRight.as_str();
            }
            ContainerStyle::Heavy => {
                // Top
                r#box += Symbol::BoxHeavyFull.as_str();

                for _i in 1..=self.size.width {
                    r#box += Symbol::BoxHeavyUp.as_str();
                }

                r#box += Symbol::BoxHeavyFull.as_str();

                r#box += "\n";

                // Middle
                for _j in 1..=self.size.height {
                    r#box += Symbol::BoxHeavyFull.as_str();

                    for _i in 1..=self.size.width {
                        r#box += " ";
                    }

                    r#box += Symbol::BoxHeavyFull.as_str();

                    r#box += "\n";
                }

                // Bottom
                r#box += Symbol::BoxHeavyFull.as_str();

                for _i in 1..=self.size.width {
                    r#box += Symbol::BoxHeavyDown.as_str();
                }

                r#box += Symbol::BoxHeavyFull.as_str();
            }
        };

        r#box
    }
}
