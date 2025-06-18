use std::{fs::OpenOptions, io::BufReader};
use std::io::Read;

use colored::*;

struct Entry {
    name: String,
    status: bool,
    commenced_date: String,
    due_date: String
}

impl Entry {
    pub fn new(
        entry_name: String, status: bool,
        commenced_date: String, due_date: String
    ) -> Self {
        Self {
            name: entry_name,
            status: status,
            commenced_date: commenced_date,
            due_date: due_date,
        }
    }

    pub fn mark_entry_line(&self) -> String {
       let state = if self.status {"[*]"} else {"[ ]"};
       format!("{}, {}\n", &self.name, state)
    }

    pub fn format_entry_line(&self, number: i8) -> String {
        let entry_name_formatted = if self.status {
            self.name.strikethrough().to_string()
        } else {
            self.name.clone()
        };
        format!("{}, {}", number, entry_name_formatted)
    }
}


struct Todo {
    items: Vec<String>,
    todo_path: String,
}

impl Todo {
    pub fn new() -> Result<Self, ()> {
        // Search for an arbitrary path in env for a todo list items
        let todo_path = match std::env::var("TODO_PATH") {
            Ok(target_path) => target_path,
            Err(_) => {
                let home_path = std::env::var("HOME").unwrap();
                let legacy_path = format!("{}/TODO", &home_path);

                match std::path::Path::new(&legacy_path).exists() {
                    true => legacy_path,
                    false => format!("{}/.qtodo", &home_path),
                }
            }
        };

        let todofile = OpenOptions::new()
            .write(true)
            .read(true)
            .create(true)
            .open(&todo_path)
            .expect(
                &format!("Could not open the file located at {}", &todo_path)
            );

        let mut todobuf = BufReader::new(&todofile);
        let mut todos = String::new();

        todobuf.read_to_string(&mut todos).map_err(|err| {
            eprintln!("Could not read the content of Buffer due to {err}")
        }).unwrap();
        let todo_items = todos.lines().map(str::to_string).collect::<Vec<String>>();

        Ok(
            Self {
                items: todo_items,
                todo_path: todo_path
            }
        )
    }
}

fn main() {
    println!("Hello, world!");
}
