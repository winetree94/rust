#[derive(Debug)]
pub struct TodoListItem {
    pub todo: String,
}

impl TodoListItem {
    pub fn new(todo: String) -> TodoListItem {
        TodoListItem { todo }
    }
}
