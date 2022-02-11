use std::error::Error;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
use std::io::BufWriter;
use std::io::Write;
use std::path::PathBuf;
use crate::TodoItem;

const PATH: &'static str = ".local/share/todo.data";

pub fn user_path() -> PathBuf {
    let mut home = PathBuf::new();
    home.push(std::env::var("HOME").unwrap());
    home.push(PATH);
    home
}

pub fn append(todo_item: TodoItem) -> Result<(), Box<dyn Error>> {
    let mut f = File::options()
        .create(true)
        .append(true)
        .open(user_path())?;

    writeln!(f, "{}", todo_item.to_string())?;
    Ok(())
}

pub fn read() -> Result<Vec<TodoItem>, Box<dyn Error>> {
    let mut items = Vec::new();

    let f = File::options()
        .read(true)
        .write(true)
        .create(true)
        .open(user_path())?;

    let reader = BufReader::new(f);

    for line in reader.lines() {
        let line = line?;
        items.push(TodoItem::parse(line));
    }

    Ok(items)
}

pub fn modify(new: TodoItem) -> Result<(), Box<dyn Error>> {
    let mut items = read()?;

    for item in items.iter_mut() {
        if item.id == new.id {
            item.name = new.name.clone();
            item.date = new.date;
            item.done = new.done;
        }
    }

    write(items)?;
    Ok(())
}

pub fn remove(id: String) -> Result<(), Box<dyn Error>> {
    let items = read()?.into_iter()
        .filter(|e| e.id != id)
        .collect();

    write(items)?;

    Ok(())
}

pub fn write(items: Vec<TodoItem>) -> Result<(), Box<dyn Error>> {
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
