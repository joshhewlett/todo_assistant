use std::collections::HashMap;

use crate::todo::todo_item::TodoItem;
use crate::error::todo_error::TodoError;

pub struct TodoStore {
    store: Vec<TodoItem>,
    longest_title: usize,
}

impl TodoStore {
    pub fn new() -> Result<TodoStore, TodoError> {
        let mut store = vec![
            TodoItem::new(String::from("Todo Number 2"), String::from("2022-02-02")),
            TodoItem::new(String::from("Todo Number 1"), String::from("2022-02-01")),
            TodoItem::new(String::from("Todo Number 3"), String::from("2022-02-03"))];

        let mut longest_title: usize = 0;
        store.iter().for_each(|item| if item.title.len() > longest_title {
            longest_title = item.title.len()
        });

        let mut result = TodoStore {
            store,
            longest_title,
        };
        result.sort_store();

        result.mark_as_done(1);

        Ok(result)
    }

    // TODO: Accept user input and pass into TodoItem::new
    pub fn create_new_todo(&mut self, new_item: TodoItem) {
        self.store.push(new_item);
        self.sort_store();
    }

    pub fn mark_as_done(&mut self, index: usize) {
        self.store[index].mark_as_done();
    }

    pub fn list_all_todos(&self) {
        self.print_store(|_| true);
    }

    pub fn list_incomplete_todos(&self) {
        self.print_store(|item: &&TodoItem| !item.complete);
    }

    pub fn list_history(&self) {
        self.print_store(|item: &&TodoItem| item.complete);
    }

    fn sort_store(&mut self) {
        self.store.sort_by(|a, b| a.due_date.cmp(&b.due_date))
    }

    fn print_store<F>(&self, filter: F)
        where
            F: FnMut(&&TodoItem) -> bool // TODO: Is a double reference necessary?
    {
        let title_divider = String::from("-").repeat(self.longest_title + 1);

        println!(" # | √ | Date due   | Title");
        println!("---|---|------------|{}", title_divider);

        for (i, val) in self.store.iter()
            .filter(filter)
            .enumerate() {
            println!(" {} {}", i, val);
        }
    }
}

fn initiate_store_from_file(path: String) {}