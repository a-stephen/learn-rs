use std::{fs::OpenOptions, io::BufReader};
use std::io::{BufWriter, Read, Write, Error};

use colored::*;


struct Entry {
    name: String,
    done: bool,
    commenced_date: String,
    due_date: String
}


impl Entry {
    pub fn new(
        entry_name: String, done: bool,
        commenced_date: String, due_date: String
    ) -> Self {
        Self {
            name: entry_name,
            done: done,
            commenced_date: commenced_date,
            due_date: due_date,
        }
    }

    pub fn mark_entry_line(&self) -> String {
       let state = if self.done {"[*]"} else {"[ ]"};
       format!("{}, {}\n", &self.name, state)
    }

    pub fn format_entry_line(&self, number: usize) -> String {
        let entry_name_formatted = if self.done {
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
             self.done,
        )
    }

    /// Function to convert string line to Entry
    ///     - Considers the length of the parsed array
    ///     - Name is an important argument, second position in entry
    ///     - Commenced date and due date can be omitted
    pub fn read_entry_line(entry_line: String) -> Result<Self, String> {
        let line_array = entry_line.split(";").map(str::to_string).collect::<Vec<String>>();
        let default_entry = Self {
            name: String::from(""),
            done: false,
            commenced_date: String::from(""),
            due_date: String::from("")
        };
        match line_array.len() {
            x if x == 4 => {
                return Ok(
                    Self {
                        name: String::from(&line_array[1]),
                        done: if &line_array[0] == "[*]" {true} else {false},
                        commenced_date: String::from(&line_array[2]),
                        due_date: String::from(&line_array[3])
                    }
                )
            }
            2 => {
                return Ok(
                    Self {
                        name: String::from(&line_array[1]),
                        done: if &line_array[0] == "[*]" {true} else {false},
                        commenced_date: String::from(""),
                        due_date: String::from("")
                    }
                )
            },
            3 => {
                return Ok(
                    Self {
                        name: String::from(&line_array[1]),
                        done: if &line_array[0] == "[*]" {true} else {false},
                        commenced_date: String::from(&line_array[2]),
                        due_date: String::from("")
                    }
                )
            }
            1 => {
                if line_array[0].starts_with("[") {
                    return Ok(default_entry)
                } else {
                    Ok(
                        Self {
                            name: String::from(&line_array[0]),
                            done: false,
                            commenced_date: String::from(""),
                            due_date: String::from("")
                        }
                    )
                }
            },
            0 => {
                eprintln!("Please consider adding at least the name of todo");
                return Ok(default_entry)
            },
            _ => Err("an ERROR occured".to_string())
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
                    let entry = line_item.expect("Entry not parsed properly").format_entry_line(number);
                    todos.push_str(&entry);
                };
                writer
                    .write_all(
                    todos.as_bytes()
                ).expect("Data can not be writtten to stdout!");
            },
            0 => {
                let empty_todo_msg = "No task for now.\nPlease consider adding\n";
                todos.push_str(empty_todo_msg);
                writer
                    .write_all(todos.as_bytes())
                    .expect("Data can not be written to stdout")
            },
            _ => todo!()
        }
    }

    /// Function to add new task to todo file
    ///     - All new entry default to fasle as done
    ///     - Commenced date defaults to the creation date
    ///     - Completed date/due date defaults to null
    pub fn add(&self, entries: &[String]) -> String {
        println!("{} entries about to be added to todo", &entries.len());
        for entry in entries.iter() {
            let new_entry = Entry::read_entry_line(String::from(entry)).map_err(|err| {
                eprintln!("An error occured: {err}")
            }).unwrap();
            println!("{:?}", new_entry.name.trim())
        };
        format!("Successfully added {} tasks", &entries.len())
    }
}

fn main() {
    let todos = Todo::new().unwrap();
    // println!("Todo items: {:?} | Todo path: {:?}", todos.items, todos.todo_path);
    let entry_add =  todos.add(&[String::from("[]; Me"), String::from("; Stephen"), String::from("; angelo")]);
    println!("{:?}", entry_add)
}
