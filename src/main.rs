pub mod color;
use color::*;

fn main() {
    let colors = [
        Color::Black,
        Color::Red,
        Color::Green,
        Color::Yellow,
        Color::Blue,
        Color::Magenta,
        Color::Cyan,
        Color::White,
    ];

    let reset = Reset;

    let mut font = Font::new(Color::Black, Color::Black);

    for color_background in colors.iter() {
        font.color_set_background(*color_background);

        for color_text in colors.iter() {
            font.color_set_text(*color_text);

            print!("{}A{}", font, reset);

            font.decoration_set(ColorDecoration::Bold);
            print!("{}A{}", font, reset);
            font.decoration_unset(ColorDecoration::Bold);

            font.decoration_set(ColorDecoration::Underline);
            print!("{}A{}", font, reset);
            font.decoration_unset(ColorDecoration::Underline);

            font.decoration_set(ColorDecoration::Reversed);
            print!("{}A{}", font, reset);
            font.decoration_unset(ColorDecoration::Reversed);

            print!(" ");
        }
        println!();
    }

}
