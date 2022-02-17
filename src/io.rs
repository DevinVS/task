use std::error::Error;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
use std::io::BufWriter;
use std::io::Write;
use std::path::PathBuf;
use crate::task::Task;

const PATH: &'static str = ".local/share/todo.data";

pub fn user_path() -> PathBuf {
    let mut home = PathBuf::new();
    home.push(std::env::var("HOME").unwrap());
    home.push(PATH);
    home
}

pub fn read() -> Result<Vec<Task>, Box<dyn Error>> {
    let mut items = Vec::new();

    let f = File::options()
        .read(true)
        .write(true)
        .create(true)
        .open(user_path())?;

    let reader = BufReader::new(f);

    for line in reader.lines() {
        let line = line?;
        items.push(Task::parse(line));
    }

    Ok(items)
}

pub fn write(items: &Vec<Task>) -> Result<(), Box<dyn Error>> {
    let f = File::options()
        .write(true)
        .truncate(true)
        .open(user_path())?;
    let mut writer = BufWriter::new(f);

    for item in items {
        writeln!(writer, "{}", item.to_string())?;
    }

    Ok(())
}
