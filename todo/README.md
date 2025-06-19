## This is a simple do app written in rust

- Steps
    - Simple Interface:
        - Define todo Entry as a struct
            ```rust
                struct Entry {
                    name: String,
                    status: bool,
                    commenced_date: String // Must be a Duration type (Will be handled later)
                    due_date: String // Must be a Duration Type as well (Will be handled later)
                }

            ```
        - Define the Todo list itself as a struct
            ```rust
                struct Todo {
                    items: Vec<String> // Vec<Entry> tbh :)
                    todo_path: String // Local copy of the todo file
                }
            ```

    - Entry Functions
        - An entry can be read as string
        - An entry can be modified

    - Todo Functions
        - Todo's can be listed
        - Todo's can be edited, sorted, modified (add & remove)
