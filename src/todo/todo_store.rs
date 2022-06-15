use crate::todo::todo_item::TodoItem;

pub struct TodoStore {
    store: Vec<TodoItem>,
}

impl TodoStore {
    pub fn new() -> TodoStore {
        TodoStore {
            store: vec![
                TodoItem::new(String::from("Todo Number 1"), String::from("2022-02-01")),
                TodoItem::new(String::from("Todo Number 2"), String::from("2022-02-02")),
                TodoItem::new(String::from("Todo Number 3"), String::from("2022-02-03"))]
        }
    }

    pub fn create_new_todo(&mut self, new_item: TodoItem) {
        self.store.push(new_item);
    }

    pub fn mark_as_done(&mut self, index: usize) {
        self.store[index].mark_as_done();
    }

    pub fn list_all_todos(&self) {
        for i in &self.store {
            println!("{}", i);
        }
    }

    pub fn list_incomplete_todos(&self) {
        for i in &self.store {
            if !i.complete {
                println!("{}", i);
            }
        }
    }

    pub fn list_history(&self) {
        for i in &self.store {
            if i.complete {
                println!("{}", i);
            }
        }

    }
}

fn initiate_store_from_file(path: String) {

}