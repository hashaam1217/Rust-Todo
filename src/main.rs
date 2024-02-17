use std::fmt;


#[derive(Debug)]
struct TodoItem {
    id: u32,
    name: String,
    completed: bool,
}

impl fmt::Display for TodoItem {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "id: {}, name: {}, completed: {}", self.id, self.name, self.completed)
    }
}

fn main() {
    println!("Hello, world!");
    let mut todo_list: Vec<TodoItem> = Vec::new();
    let item1 = TodoItem { id: 1, name: String::from("Buy milk"), completed: false };
    let item2 = TodoItem { id: 2, name: String::from("Walk the dog"), completed: false };
    todo_list.push(item1);
    todo_list.push(item2);
    println!("{:?}\n", todo_list);
}
