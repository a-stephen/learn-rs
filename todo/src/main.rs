use std::{fs::OpenOptions, io::BufReader};
use std::io::{BufWriter, Read, Write};

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

    pub fn format_entry_line(&self, number: usize) -> String {
        let entry_name_formatted = if self.status {
            self.name.strikethrough().to_string()
        } else {
            self.name.clone()
        };
        format!(
             "{}, {}, {}, {}, {}",
             number,
             entry_name_formatted,
             self.commenced_date,
             self.due_date,
             self.status,
        )
    }

    pub fn read_entry_line(entry_line: String) -> Self {
        let line_array = entry_line.split(";").map(str::to_string).collect::<Vec<String>>();
        println!("{:?}", line_array);
        let status = if line_array[0] == "[*]" { true } else { false };
        let name = &line_array[1];
        let commenced_date = &line_array[2];
        let due_date = &line_array[3];
        return Self {
           name: name.to_string(),
           status: status,
           commenced_date: commenced_date.to_string(),
           due_date: due_date.to_string()
        }
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
    pub fn list(&self) {
        let stdout = std::io::stdout();
        let mut writer = BufWriter::new(stdout);
        let mut todos = String::new();
        match self.items.len() {
            x if x > 0 => {
                for (number, task) in self.items.iter().enumerate() {
                    let line_item = Entry::read_entry_line(task.to_string());
                    let number = number + 1;
                    let entry = line_item.format_entry_line(number);
                    todos.push_str(&entry);
                };
                writer
                    .write_all(
                    todos.as_bytes()
                ).expect("Data can not be writtten to stdout!");
            },
            0 => {
                let empty_todo_msg = "No task for now.\n Please consider adding";
                todos.push_str(empty_todo_msg);
                writer
                    .write_all(todos.as_bytes())
                    .expect("Data can not be written to stdout")
            },
            _ => todo!()
            // Err(_) => writer.write_all("No data to write".as_bytes());
        }
    }
}

fn main() {
    let entry_1 = Entry::new(
        "Laundry".to_string(),
        false,
        "21/06/2024".to_string(),
        "25/06/2025".to_string()
    );
    let entry = "[ ]; do laundry; 21/09/190; 21/09/1009".to_string();
    let entry_one =  Entry::read_entry_line(entry);
    println!("{:?}", entry_one.format_entry_line(7));
}
