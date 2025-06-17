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
}


struct Todo {
    items: Vec<String>,
    todo_path: String,
    todo_bk: String
}

impl Todo {
    pub fn new(path: String) -> Result<Self, ()> {
        Ok(Self {
            items: vec![],
            todo_path: path,
            todo_bk: String::new(),
        })
    }
}

fn main() {
    println!("Hello, world!");
}
