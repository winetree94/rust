use std::{fs::File, io::Read};

#[derive(Debug)]
pub struct Todo {
    pub name: String,
    pub category: Option<String>,
    pub completed: bool,
}

impl Todo {
    fn new(name: String, category: Option<String>) -> Todo {
        Todo {
            name,
            category,
            completed: false,
        }
    }
}

pub struct TodoApp {
    pub todos: Vec<Todo>,
}

impl TodoApp {
    fn new() -> TodoApp {
        match File::open("db.txt") {
            Ok(mut file) => {
                let mut contents = String::new();
                file.read_to_string(&mut contents).expect("Failed to read file");
                contents.lines().enumerate().for_each(|(index, line)| {
                    line.split(",");
                    println!("{index} {}", line);
                });
            }
            Err(error) => {
                println!("Error: {}", error);
            }
        }
        TodoApp { todos: vec![] }
    }

    fn start(&mut self) -> Result<(), std::io::Error> {
        loop {
            println!("---------------------");
            println!("1. Add todo");
            println!("2. List todos");
            println!("3. Mark todo as completed");
            println!("4. Exit");
            println!("---------------------");
            let mut input = String::new();
            std::io::stdin().read_line(&mut input)?;

            let _ = match input.trim().parse() {
                Ok(1) => self.add_todo_prompt(),
                Ok(2) => self.list_todo_prompt(),
                Ok(3) => {
                    self.list_todo_prompt()
                }
                Ok(4) => break,
                _ => {
                    println!("Invalid input, try again.");
                    Ok(())
                }
            };
        }
        Ok(())
    }

    fn add_todo_prompt(&mut self) -> Result<(), std::io::Error> {
        println!("---------------------");
        println!("Enter the name of the todo:");
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();
        let todo = Todo::new(input.trim().to_string(), None);
        self.add_todo(todo);
        println!("Todo added successfully");
        println!("---------------------");
        Ok(())
    }

    fn list_todo_prompt(&mut self) -> Result<(), std::io::Error> {
        println!("---------------------");
        self.todos.iter().enumerate().for_each(|(index, todo)| {
            println!("{index}: Name: {}", todo.name);
            println!("   Category: {:?}", todo.category);
            println!("   Completed: {:?}", todo.completed);
        });
        println!("---------------------");
        Ok(())
    }

    fn add_todo(&mut self, todo: Todo) {
        self.todos.push(todo);
    }
}

fn main() -> Result<(), std::io::Error> {
    let mut app = TodoApp::new();
    app.start()?;
    Ok(())
}
