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

use chrono::{DateTime, Days, Local, NaiveDate, NaiveDateTime, NaiveTime, TimeZone};

use std::{fs::OpenOptions, io::Read};
use std::io::Write;

use serde_derive::{Serialize, Deserialize};

use std::env;

const SCREEN_WIDTH: usize = 64;
const SCREEN_HEIGHT: usize = 16;
const APP_TITLE: &'static str = "Taskori";

#[derive(Serialize, Deserialize, Debug)]
struct Event {
    name: String,
    start: String,
    end: String,
}
#[derive(Serialize, Deserialize, Debug)]
struct Events {
    events: Vec<Event>,
}

fn main() {
    let time_date: DateTime<Local> = chrono::offset::Local::now();

    let mut events_file_read = OpenOptions::new().read(true).open("events.json").unwrap();
    let mut events_content = String::new();
    events_file_read.read_to_string(&mut events_content).unwrap();
    let mut events: Events = serde_json::from_str(&events_content).unwrap();

    let args: Vec<String> = env::args().collect();
    if args.len() > 1 {
        enum Methods {
            Add,
        }
        let mut method: Option<Methods> = None;

        enum Modes {
            Name,
            Date,
            TimeStart,
            TimeEnd,
        }
        let mut mode: Option<Modes> = None;

        let mut name: Option<String> = None;
        let mut date: Option<String> = None; let date_parsed: NaiveDate;

        let mut time_start: Option<String> = None; let time_start_parsed: NaiveTime;
        let mut time_end: Option<String> = None; let time_end_parsed: NaiveTime;

        for arg in args.iter().skip(1) {
            if let Some(mode) = &mode {
                match mode {
                    Modes::Name => { name = Some(arg.clone()); }
                    Modes::Date => { date = Some(arg.clone()); }
                    Modes::TimeStart => { time_start = Some(arg.clone()); }
                    Modes::TimeEnd => { time_end = Some(arg.clone()); }
                }
            }
            mode = None;

            match arg.as_str() {
                "add" => { method = Some(Methods::Add); }

                "-n" | "--name" => { mode = Some(Modes::Name); }

                "-d" | "--date" => { mode = Some(Modes::Date); }

                "-s" | "--time-start" => { mode = Some(Modes::TimeStart); }
                "-e" | "--time-end" => { mode = Some(Modes::TimeEnd); }

                _ => ()
            }
        }

        if method.is_none() { eprintln!("Method required!"); return; }

        let name = if let Some(name) = name { name } else { eprintln!("Name required!"); return; };

        if date.is_none() {
            date_parsed = NaiveDate::parse_from_str(format!("{}", time_date.format("%d.%m.%Y")).as_str(), "%d.%m.%Y").unwrap();
        } else {
            date_parsed = NaiveDate::parse_from_str(&date.unwrap(), "%d.%m.%Y").unwrap();
        }

        if time_start.is_none() { eprintln!("Start Time required!"); return; }
        else {
            time_start_parsed = NaiveTime::parse_from_str(&time_start.unwrap(), "%H:%M").unwrap();
        }

        if time_end.is_none() {
            time_end_parsed = NaiveTime::parse_from_str("23:59:59", "%H:%M:%S").unwrap();
        } else {
            time_end_parsed = NaiveTime::parse_from_str(&time_end.unwrap(), "%H:%M").unwrap();
        }

        let date_time_start: NaiveDateTime = date_parsed.and_time(time_start_parsed);
        let date_time_end: NaiveDateTime = date_parsed.and_time(time_end_parsed);

        let event_name: String = name;
        let event_start: DateTime<Local> = Local.from_local_datetime(&date_time_start).unwrap();
        let event_end: DateTime<Local> = Local.from_local_datetime(&date_time_end).unwrap();


        events.events.push(Event{name: event_name, start: format!("{:?}", event_start), end: format!("{:?}", event_end)});

        let mut events_file_write = OpenOptions::new().write(true).truncate(true).open("events.json").unwrap();
        events_file_write.write_all(serde_json::to_string(&events).unwrap().as_bytes()).unwrap();

        return;
    }

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
