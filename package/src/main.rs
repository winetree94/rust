mod mytodo;

pub use mytodo::{item, list};

fn main() {
    println!("Hello, world!");

    let mut list = list::TodoList::new();

    let item = item::TodoListItem::new(String::from("Hello World"));

    list.items.push(item);

    println!("item: {:#?}", &list);
}
