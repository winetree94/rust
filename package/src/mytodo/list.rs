use super::item;

#[derive(Debug)]
pub struct TodoList {
    pub items: Vec<item::TodoListItem>,
}

impl TodoList {
    pub fn new() -> TodoList {
        TodoList { items: vec![] }
    }
}
