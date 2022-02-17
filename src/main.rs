use std::io::stdin;
use std::io::stdout;
use std::io::Write;

use clap::Parser;
use clap::AppSettings;
use clap::Subcommand;

#[derive(Parser, Debug)]
#[clap(
    author="Devin Vander Stelt",
    version="1.0",
    about="A Simple CLI Todo App",
    long_about=None,
)]
#[clap(global_setting(AppSettings::PropagateVersion))]
#[clap(global_setting(AppSettings::UseLongFormatForHelpSubcommand))]
struct Args {
    /// Command to run, defaults to list
    #[clap(subcommand)]
    command: Option<Command>,
}

#[derive(Subcommand, Debug)]
enum Command {
    /// List out current tasks
    List {
        /// Print out all todo items
        #[clap(short, long)]
        all: bool
    },
    /// Add a task to be completed (interactive)
    Add,
    /// Mark a task as complete
    Done {
        /// Id of the task to mark as done
        id: String,
    },
    /// Remove a task
    Rm {
        /// Id of the task to remove
        id: String,
    }
}

fn main() {
    let args = Args::parse();

    match args.command {
        Some(Command::Add) => {
            let name = prompt("Name: ").unwrap();
            let datetime = prompt("Date: ").unwrap();
            task::add(name.clone(), datetime);
            println!("Added task '{name}'\n");
            task::list(false)
        },
        Some(Command::List { all }) => {
            task::list(all)
        }
        Some(Command::Done { id }) => {
            task::done(id.clone());
            println!("Task {id} completed!\n");
            task::list(false)
        }
        Some(Command::Rm { id }) => {
            task::remove(id.clone());
            println!("Task {id} removed.\n");
            task::list(false)
        }
        _ => {
            task::list(false)
        }
    }
}

fn prompt(prompt: &str) -> Option<String> {
    print!("{}", prompt);
    stdout().flush().unwrap();
    let mut buf = String::new();
    stdin().read_line(&mut buf).unwrap();

    if buf.trim().is_empty() {
        None
    } else {
        Some(buf.trim().into())
    }
}
