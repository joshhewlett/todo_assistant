use std::collections::HashMap;
use std::io;

use crate::todo::todo_item::TodoItem;
use crate::error::todo_error::TodoError;

#[derive(Debug)]
pub struct TodoStore {
    store: Vec<TodoItem>,
    longest_title: usize,
}

impl TodoStore {
    pub fn new() -> Result<TodoStore, TodoError> {

        let mut result = TodoStore {
            store: Vec::new(),
            longest_title: 0,
        };
        result.sort_store();

        Ok(result)
    }

    pub fn create_new_todo(&mut self) -> Result<(), TodoError> {
        println!("Enter a new Todo Item:");
        println!("Format: 'YYYY-MM-DD {{Title}}");

        let mut new_todo = String::new();
        io::stdin().read_line(&mut new_todo)
            .map_err(|err| TodoError::new(
                String::from("Failed to read line."),
                Box::new(err)))?;

        self.add_item(TodoItem::new(new_todo)?);
        self.sort_store();
        Ok(())
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

    fn add_item(&mut self, new_item: TodoItem) {
        if new_item.title.len() > self.longest_title {
            self.longest_title = new_item.title.len();
        }
        self.store.push(new_item);
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