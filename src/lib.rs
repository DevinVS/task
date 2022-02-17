use chrono::NaiveTime;

use crate::util::get_random_id;
use crate::task::{Task, TaskManager};

mod io;
mod task;
mod util;

fn print_header(width: u16) {
    let buffer = (0..width-30).map(|_| ' ').collect::<String>();
    let sep = (0..width).map(|_| '-').collect::<String>();
    println!("Id   : Name{buffer} | Date            ");
    println!("{sep}");
}

pub fn list(all: bool) {
    let tm = TaskManager::load();

    match tm {
        Err(e) => eprintln!("{e}"),
        Ok(tm) => {
            let width = util::get_term_width();

            print_header(width);
            if all {
                let mut tasks: Vec<&Task> = tm.all().collect();
                tasks.sort_by_key(|t| t.date);

                for task in tasks {
                    println!("{}", task.pretty(width));
                }
            } else {
                let mut tasks: Vec<&Task> = tm.unfinished().collect();
                tasks.sort_by_key(|t| t.date);
                for task in tasks {
                    println!("{}", task.pretty(width));
                }
            }
        }
    }
}

pub fn add(name: String, datetime: String) {
    let tm = TaskManager::load();

    match tm {
        Err(e) => eprintln!("{}", e),
        Ok(mut tm) => {
            let date = fuzzydate::parse_with_default_time(&datetime, NaiveTime::from_hms(23,59,59));

            if date.is_err() {
                eprintln!("Invalid Date: {}", date.err().unwrap());
                return;
            }

            let date = date.unwrap();

            let task = Task::new(get_random_id(), name, date);
            tm.add(task);
            match tm.sync() {
                Err(e) => eprintln!("{}", e),
                Ok(_) => eprintln!("Task added successfully")
            }
        }
    }
}

pub fn done(id: String) {
    let tm = TaskManager::load();

    match tm {
        Err(e) => eprintln!("{}", e),
        Ok(mut tm) => {
            tm.done(id);
            match tm.sync() {
                Err(e) => eprintln!("{}", e),
                Ok(_) => eprintln!("Task finished!")
            }
        }
    }
}

pub fn remove(id: String) {
    let tm = TaskManager::load();

    match tm {
        Err(e) => eprintln!("{}", e),
        Ok(mut tm) => {
            tm.remove(id);
            match tm.sync() {
                Err(e) => eprintln!("{}", e),
                Ok(_) => eprintln!("Task Finished")
            }
        }
    }
}
