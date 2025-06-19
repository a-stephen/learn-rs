use std::{fs::OpenOptions, io::BufReader};
use std::io::{BufWriter, Read, Write, Error};

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

        let (todos, todo_path) = Todo::read_todos(todo_path).map_err(|err| {
                eprintln!("ERROR: {err}");
            }
        ).unwrap();

        Ok(
            Self {
                items: todos,
                todo_path: todo_path
            }
        )
    }

    pub fn read_todos(path: String) -> Result<(Vec<String>, String), Error> {
        let todo_file = OpenOptions::new()
            .write(true)
            .read(true)
            .create(true)
            .open(&path)
            .expect(
                &format!("Could not open the file store at location: {}", &path)
            );

        let mut file_buf = BufReader::new(&todo_file);
        let mut file_content = String::new();

        file_buf.read_to_string(&mut file_content).map_err(|err| {
            eprintln!("ERROR: {err}")
        }).unwrap();

        match file_content.lines().collect::<Vec<_>>().len() {
            x if x > 0 => {
                let todos = file_content
                    .lines()
                    .map(str::to_string)
                    .collect::<Vec<String>>();
                Ok(
                    (todos, path)
                )
            },

            0 => Ok((vec![], path)),
            _ => Ok((vec![], path))
        }

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
        }
    }
}

fn main() {
    let todos = Todo::new().unwrap();
    println!("Todo items: {:?} | Todo path: {:?}", todos.items, todos.todo_path);
}
