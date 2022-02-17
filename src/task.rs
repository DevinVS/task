use std::error::Error;
use crate::io;
use std::iter::Peekable;
use chrono::{NaiveDateTime as DateTime, Local};

/// Manages creating, reading, and altering tasks
pub struct TaskManager {
    /// All tasks
    tasks: Vec<Task>,
    /// Keeps track if we have made any destructive changes to the tasks
    dirty: bool
}

impl TaskManager {
    /// Load the tasks from the task file
    pub fn load() -> Result<TaskManager, Box<dyn Error>> {
        Ok(Self {
            tasks: io::read()?,
            dirty: false
        })
    }

    /// Return iterator over all tasks
    pub fn all(&self) -> impl Iterator<Item=&Task> {
        self.tasks.iter()
    }

    /// Return iterator over all unfinished tasks
    pub fn unfinished(&self) -> impl Iterator<Item=&Task> {
        self.tasks.iter().filter(|t| !t.done)
    }

    /// Remove a task
    pub fn remove(&mut self, task_id: String) {
        if let Some(index) = self.get_index(task_id) {
            self.tasks.remove(index);
            self.dirty = true;
        }
    }

    pub fn get_index(&self, task_id: String) -> Option<usize> {
        self.tasks.iter()
            .enumerate()
            .find(|(_, t)| t.id==task_id)
            .map(|(i, _)| i)
    }

    /// Add a task
    pub fn add(&mut self, task: Task) {
        self.tasks.push(task);
        self.dirty = true;
    }

    /// Mark a task as done
    pub fn done(&mut self, task_id: String) {
        if let Some(index) = self.get_index(task_id) {
            self.tasks[index].done = true;
            self.dirty = true;
        }
    }

    /// Sync manager to disk
    pub fn sync(&mut self) -> Result<(), Box<dyn Error>> {
        if self.dirty {
            io::write(&self.tasks)?;
        }

        self.dirty = false;

        Ok(())
    }
}

#[derive(Debug, Clone)]
pub struct Task {
    pub id: String,
    pub done: bool,
    pub name: String,
    pub date: DateTime,
}

impl Task {
    pub fn new(id: String, name: String, date: DateTime) -> Self {
        Self {
            id,
            name,
            date,
            done: false,
        }
    }

    pub fn to_string(&self) -> String {
        format!(
            "{}|{}|{}|{}",
            self.id,
            Self::escape(self.name.clone()),
            self.date.timestamp(),
            self.done
        )
    }

    fn escape(s: String) -> String {
        s.replace("|", "\\|")
    }

    pub fn parse(s: String) -> Self {
        let mut chars = s.chars().peekable();

        // Parse id
        let id = Self::parse_delimited(&mut chars);

        // Parse name
        let name = Self::parse_delimited(&mut chars);

        // Parse date
        let timestamp = Self::parse_delimited(&mut chars).parse::<i64>().unwrap();
        let date = DateTime::from_timestamp(timestamp, 0);

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

    pub fn pretty(&self, width: u16) -> String {
        let diff = self.date.signed_duration_since(
            Local::now().naive_local()
        );

        let color = if diff.num_days() < 0 {
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
            self.date.format("%b %d, %l:%M %P"),
            width = (width-26) as usize
        )
    }
}

impl PartialEq for Task {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}
impl Eq for Task {}
