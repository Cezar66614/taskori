pub mod definitions;
use definitions::*;

pub mod symbol;

pub mod container;
use container::*;

pub mod font;
use font::*;

pub mod rich_string;
use rich_string::*;

pub mod canvas;
use canvas::*;

pub mod screen;
use screen::*;

const SCREEN_WIDTH: usize = 64;
const SCREEN_HEIGHT: usize = 16;

use std::{thread, time};

fn main() {
    let mut screen = Screen::new(Size{width: SCREEN_WIDTH, height: SCREEN_HEIGHT});
    let mut canvas = Canvas::new(Size{width: SCREEN_WIDTH / 2, height: SCREEN_HEIGHT});
    let mut canvas_2 = Canvas::new(Size{width: SCREEN_WIDTH / 2, height: SCREEN_HEIGHT});

    
    let container = Container::new(Size { width: SCREEN_WIDTH / 2 - 2, height: SCREEN_HEIGHT - 2 }, ContainerStyle::Double);
    let container = container.as_str();

    canvas.add_string(&container, Position{x: 0, y: 0});

    let font = Font::new(Color::Yellow, Color::Magenta);
    let text = RichString::new("Hello, World!", font);
    canvas.add_rich_string(&text, Position{x: 1, y: 1});

    let font = Font::new(Color::Blue, Color::White);
    let text = RichString::new("Hello,\nWorld!", font);
    canvas.add_rich_string(&text, Position{x: 1, y: 2});

    screen.add_canvas(&canvas, Position{x: 0, y: 0});


    let mut style = 0;

    loop {
        let container: Container;
        if style == 0 {
            container = Container::new(Size { width: SCREEN_WIDTH / 2 - 2, height: SCREEN_HEIGHT - 2 }, ContainerStyle::Single);
        } else {
            container = Container::new(Size { width: SCREEN_WIDTH / 2 - 2, height: SCREEN_HEIGHT - 2 }, ContainerStyle::Heavy);
        }
        let container = container.as_str();

        canvas_2.add_string(&container, Position{x: 0, y: 0});

        let text = String::from("Hello, World!");
        canvas_2.add_string(&text, Position{x: 1, y: 1});

        screen.add_canvas(&canvas_2, Position{x: SCREEN_WIDTH / 2, y: 0});


        screen.print();

        thread::sleep(time::Duration::from_secs(1));

        style = !style;
    }

    /* Container Test
    let container_single = Container::new(Size { width: 3, height: 1 }, ContainerStyle::Single);
    let container_double = Container::new(Size { width: 6, height: 2 }, ContainerStyle::Double);
    let container_heavy = Container::new(Size { width: 9, height: 3 }, ContainerStyle::Heavy);

    println!("{}", container_single.as_str());
    println!("{}", container_double.as_str());
    println!("{}", container_heavy.as_str());
    */

    /* Loading Bar test
    println!("Loading...");

    let mut width: i32;
    let mut bar = String::new();

    for i in 1..=100 {
        thread::sleep(time::Duration::from_millis(100));

        print!("\r");

        print!("{}", i);

        width = i / 4;

        bar.clear();

        bar += "[";
        for _j in 0..width { bar += "#"; }
        for _j in 0..(25-width) { bar += " "; }
        bar += "]";

        print!("{}", bar);
        io::stdout().force_print().unwrap();
    }
    println!("");
    */

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
