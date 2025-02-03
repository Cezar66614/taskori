pub mod font;
use font::*;

pub mod symbol;

pub mod container;
use container::*;

fn main() {
    let container_single = Container::new(Size { width: 3, height: 1 }, ContainerStyle::Single);
    let container_double = Container::new(Size { width: 6, height: 2 }, ContainerStyle::Double);
    let container_heavy = Container::new(Size { width: 9, height: 3 }, ContainerStyle::Heavy);

    println!("{}", container_single.as_str());
    println!("{}", container_double.as_str());
    println!("{}", container_heavy.as_str());

    /* Font test
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
    */
}
