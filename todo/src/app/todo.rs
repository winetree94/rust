#[derive(Debug)]
pub struct Todo {
    pub name: String,
    pub category: Option<String>,
    pub completed: bool,
}

impl Todo {
    pub fn new(name: String, category: Option<String>, completed: bool) -> Todo {
        Todo {
            name,
            category,
            completed,
        }
    }
}
