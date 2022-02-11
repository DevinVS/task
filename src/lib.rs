mod io;
use std::iter::Peekable;

use chrono::DateTime;
use chrono::Datelike;
use chrono::Local;
use chrono::Timelike;
use nix::ioctl_read_bad;
use nix::libc::TIOCGWINSZ;
use nix::libc::winsize;
use rand::distributions::Alphanumeric;
use rand::prelude::*;

ioctl_read_bad!(tiocgwinsz, TIOCGWINSZ, winsize);
/// Get the current width of the terminal window
fn get_term_width() -> u16 {
    unsafe {
        let mut data: winsize = winsize { ws_row: 0, ws_col: 0, ws_xpixel: 0, ws_ypixel: 0 };
        tiocgwinsz(1, &mut data as *mut winsize).unwrap();
        data.ws_col
    }
}

/// Get a randomly generated, 4 character id
fn get_random_id() -> String {
    let mut rng = rand::thread_rng();

    std::iter::repeat(())
        .map(|()| rng.sample(Alphanumeric))
        .map(char::from)
        .take(4)
        .collect::<String>()
        .to_lowercase()
}

/// Get the id for a new task, ensuring there are no duplicates
fn next_id() -> String {
    let items = io::read().unwrap();

    let mut id = get_random_id();
    while items.iter().any(|i| i.id==id) {
        id = get_random_id();
    }

    id
}

#[derive(Debug, Clone)]
pub struct TodoItem {
    pub id: String,
    pub done: bool,
    pub name: String,
    pub date: DateTime<Local>,
}

impl TodoItem {
    fn new(name: String, date_string: String, time_string: String) -> Self {

        Self {
            id: next_id(),
            done: false,
            name,
            date: Self::parse_date(date_string, time_string)
        }
    }

    fn parse_date(date: String, time: String) -> DateTime<Local> {
        let mut time_split = time.split(" ");
        let mut min_split = time_split.next().unwrap().split(":");
        let mut date_split = date.split("/");

        let pm = time_split.next().unwrap().to_lowercase() == "pm";
        let month = date_split.next().unwrap().parse::<u32>().unwrap();
        let day = date_split.next().unwrap().parse::<u32>().unwrap();
        let hour = if pm {
            12
        } else {
            0
        } + min_split.next().unwrap().parse::<u32>().unwrap();
        let min = min_split.next().unwrap().parse::<u32>().unwrap();

        let date = Local::now()
            .with_month(month).unwrap()
            .with_day(day).unwrap()
            .with_hour(hour).unwrap()
            .with_minute(min).unwrap();

        date
    }

    fn to_string(&self) -> String {
        format!(
            "{}|{}|{}|{}",
            self.id,
            Self::escape(self.name.clone()),
            self.date.to_rfc2822(),
            self.done
        )
    }

    fn escape(s: String) -> String {
        s.replace("|", "\\|")
    }

    fn parse(s: String) -> Self {
        let mut chars = s.chars().peekable();

        // Parse id
        let id = Self::parse_delimited(&mut chars);

        // Parse name
        let name = Self::parse_delimited(&mut chars);

        // Parse date
        let date = DateTime::parse_from_rfc2822(
            &Self::parse_delimited(&mut chars)
        ).unwrap().with_timezone(&Local);

        // Parse done
        let done = Self::parse_delimited(&mut chars).parse::<bool>().unwrap();

        Self {id, name, date, done}
    }

    fn parse_delimited(chars: &mut Peekable<impl Iterator<Item=char>>) -> String {
        let mut stack = String::new();

        while let Some(c) = chars.next() {
            let next = chars.peek();

            if c=='\\' && next==Some(&'|') {
                stack.push('|');
                chars.next();
                continue;
            }

            if c=='|' {
                break;
            }

            stack.push(c);
        }

        stack
    }

    fn pretty(&self, width: u16) -> String {
        let diff = self.date - Local::now();
        let color = if diff.num_seconds() <= 0 {
            "\x1B[1;31m"
        } else if diff.num_days() <= 1 {
            "\x1B[1;33m"
        } else {
            "\x1B[1;32m"
        };


        format!(
            "{}{:4} : {:width$} | {}\x1B[0m",
            color,
            self.id,
            self.name,
            self.date.format("%b %d, %I:%M %p"),
            width = (width-26) as usize
        )
    }
}

fn print_header(width: u16) {
    println!("Tasks:");
    print!("Id   : Task Name");

    for _ in 0..width-35 {
        print!(" ");
    }

    println!(" | Due Date");

    for _ in 0..width {
        print!("-");
    }
    print!("\n");
}

pub fn list(all: bool) {
    let width = get_term_width();
    print_header(width);

    match io::read() {
        Err(e) => println!("{e}"),
        Ok(mut items) => {
            items.sort_by_key(|a| a.date);
            for item in items {
                if !item.done || all {
                    println!("{}", item.pretty(width));
                }
            }
        }
    }
}

pub fn add(name: String, date: String, time: String) {
    let item = TodoItem::new(name, date, time);
    match io::append(item) {
       Err(e) => println!("{e}"),
        Ok(_) => {}
    }
}

pub fn done(id: String) {
    let mut new = io::read().unwrap().iter().find(|e| e.id==id).unwrap().clone();
    new.done = true;
    io::modify(new).unwrap();
}

pub fn remove(id: String) {
    io::remove(id).unwrap();
}
