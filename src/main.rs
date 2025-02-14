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

use chrono::{DateTime, Local, Days};

const SCREEN_WIDTH: usize = 64;
const SCREEN_HEIGHT: usize = 16;
const APP_TITLE: &'static str = "Taskori";

fn main() {
    let mut screen = Screen::new(Size{width: SCREEN_WIDTH, height: SCREEN_HEIGHT});


    let mut page_main_canvas = Canvas::new(Size{width: SCREEN_WIDTH, height: SCREEN_HEIGHT});

    let page_main_container = Container::new(Size{width: SCREEN_WIDTH, height: SCREEN_HEIGHT}, ContainerStyle::Double).as_str();
    page_main_canvas.add_string(&RichString::new(&page_main_container, DEFAULT_FONT), Position{x: 0, y: 0});

    let mut page_main_title_font = Font::new(DEFAULT_COLOR_BACKGROUND, Color::Magenta); page_main_title_font.decoration_set(ColorDecoration::Bold);
    let page_main_title = APP_TITLE;
    page_main_canvas.add_string(&RichString::new(&page_main_title, page_main_title_font), Position{x: SCREEN_WIDTH / 2 - page_main_title.len() / 2, y: 1});


    let mut page_main_today_canvas = Canvas::new(Size{width: 29, height: 13});

    let page_main_today_font = Font::new(DEFAULT_COLOR_BACKGROUND, Color::Yellow);
    let mut page_main_today_title_font = page_main_today_font; page_main_today_title_font.decoration_set(ColorDecoration::Bold);

    let page_main_today_container = Container::new(Size{width: 29, height: 13}, ContainerStyle::Single).as_str();
    page_main_today_canvas.add_string(&RichString::new(&page_main_today_container, page_main_today_font), Position{x: 0, y: 0});

    page_main_today_canvas.add_string(&RichString::new("Today", page_main_today_title_font), Position{x: 1, y: 0});


    let mut page_main_future_canvas = Canvas::new(Size{width: 30, height: 8});

    let page_main_future_font = Font::new(DEFAULT_COLOR_BACKGROUND, Color::Blue);
    let mut page_main_future_title_font = page_main_future_font; page_main_future_title_font.decoration_set(ColorDecoration::Bold);

    let page_main_future_container = Container::new(Size{width: 30, height: 8}, ContainerStyle::Single).as_str();
    page_main_future_canvas.add_string(&RichString::new(&page_main_future_container, page_main_future_font), Position{x: 0, y: 0});

    page_main_future_canvas.add_string(&RichString::new("Future", page_main_future_title_font), Position{x: 1, y: 0});


    let mut page_main_week_canvas = Canvas::new(Size{width: 30, height: 5});

    let page_main_week_font = Font::new(DEFAULT_COLOR_BACKGROUND, Color::Red);
    let mut page_main_week_title_font = page_main_week_font; page_main_week_title_font.decoration_set(ColorDecoration::Bold);

    let page_main_week_container = Container::new(Size{width: 30, height: 5}, ContainerStyle::Single).as_str();
    page_main_week_canvas.add_string(&RichString::new(&page_main_week_container, page_main_week_font), Position{x: 0, y: 0});

    page_main_week_canvas.add_string(&RichString::new("Week", page_main_week_title_font), Position{x: 1, y: 0});


    let page_main_week_day_container = Container::new(Size{width: 4, height: 3}, ContainerStyle::Single).as_str();

    let page_main_week_day_font = Font::new(DEFAULT_COLOR_BACKGROUND, Color::Cyan);

    let time_date: DateTime<Local> = chrono::offset::Local::now();

    let date_month = format!("{}", time_date.format("%B"));

    page_main_week_canvas.add_string(&RichString::new(&date_month, Font::new(DEFAULT_COLOR_BACKGROUND, Color::Green)), Position{x: 20 + (9 - date_month.len()), y: 0});

    let week_days = ["M", "T", "W", "T", "F", "S", "S"];
    let week_day_current = format!("{}", time_date.format("%u")).parse::<usize>().unwrap();

    for i in 0..7 {
        let font: Font;
        if i + 1 != week_day_current {
            font = page_main_week_font;
        } else {
            font = page_main_week_day_font;
        }

        page_main_week_canvas.add_string(&RichString::new(&page_main_week_day_container, font), Position{x: 1 + 4 * i, y: 1});
        page_main_week_canvas.add_string(&RichString::new(week_days[i], font), Position{x: 2 + 4 * i, y: 1});

        if i + 1 <= week_day_current {
            page_main_week_canvas.add_string(&RichString::new(&format!("{}", time_date.checked_sub_days(Days::new((week_day_current - i - 1) as u64)).unwrap().format("%d")), font), Position{x: 2 + 4 * i, y: 2});
        } else {
            page_main_week_canvas.add_string(&RichString::new(&format!("{}", time_date.checked_add_days(Days::new((i - week_day_current + 1) as u64)).unwrap().format("%d")), font), Position{x: 2 + 4 * i, y: 2});
        }
    }


    screen.add_canvas(&page_main_canvas, Position{x: 0, y: 0});
    screen.add_canvas(&page_main_today_canvas, Position{x: 2, y: 2});
    screen.add_canvas(&page_main_future_canvas, Position{x: 32, y: 2});
    screen.add_canvas(&page_main_week_canvas, Position{x: 32, y: 10});

    screen.print();
}
