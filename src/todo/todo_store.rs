use std::fs::File;
use std::io;
use std::io::{BufReader, Read, Write};
use serde::{Serialize, Deserialize};

use crate::todo::todo_item::{TodoItem, TodoItemSerializable};
use crate::error::todo_error::TodoError;

#[derive(Debug, Serialize, Deserialize)]
struct TodoStoreSerializable {
    store: Vec<TodoItemSerializable>,
}

pub struct TodoStore {
    store: Vec<TodoItem>,
    longest_title_length: usize,
}

static PERSISTENCE_STORE_FILENAME: &str = "todo_store_data.json";

impl TodoStore {
    pub fn new_from_persistence() -> Result<TodoStore, TodoError> {

        // Open and read persistence store file contents
        let persistence_store = File::options()
            .read(true)
            .create(true)
            .write(true)
            .open(PERSISTENCE_STORE_FILENAME)
            .unwrap();

        let mut persistence_store_contents = String::new();
        BufReader::new(persistence_store).read_to_string(&mut persistence_store_contents).unwrap();

        // Deserialize persistence store
        let store_dto: TodoStoreSerializable = serde_json::from_str(&persistence_store_contents)
            .map_err(|err| TodoError::new(
                String::from("Error reading persistence file. Data is likely corrupted."),
                Box::new(err),
            ))?;

        // Create TodoItems from TodoItemSerializables
        let todo_items: Vec<TodoItem> = store_dto.store.into_iter()
            .map(|item| TodoItem::deserialize(item))
            .collect::<Result<Vec<TodoItem>, TodoError>>()?;

        // Calculate longest title length
        let mut longest_title_length = 0;
        todo_items.iter()
            .for_each(|item| if item.title.len() > longest_title_length {
                longest_title_length = item.title.len();
            });

        Ok(TodoStore {
            store: todo_items,
            longest_title_length,
        })
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
        Ok(())
    }

    pub fn mark_as_done(&mut self, index: usize) {
        self.store[index].mark_as_done();
        self.persist_data();
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
        if new_item.title.len() > self.longest_title_length {
            self.longest_title_length = new_item.title.len();
        }

        self.store.push(new_item);
        self.sort_store();
        self.persist_data();
    }

    fn sort_store(&mut self) {
        self.store.sort_by(|a, b| a.due_date.cmp(&b.due_date))
    }

    // TODO: Think about ways to optimize this.. Can we append data? How do we edit existing data?
    //   Maybe I can create a living file of appended "actions". On quit, the store is persisted and
    //   the action list is deleted. If on startup, that file exists, recreate the state
    fn persist_data(&self) {
        let store: Vec<TodoItemSerializable> = self.store.iter()
            .map(TodoItem::to_serializable)
            .collect();

        let store_dto = TodoStoreSerializable {
            store
        };

        let store_dto_json = serde_json::to_string_pretty(&store_dto).unwrap();
        let mut persistence = File::create(PERSISTENCE_STORE_FILENAME).unwrap();
        persistence.write_all(store_dto_json.as_bytes()).unwrap();
    }

    fn print_store<F>(&self, filter: F)
        where
            F: FnMut(&&TodoItem) -> bool // TODO: Is a double reference necessary?
    {
        let title_divider = String::from("-").repeat(self.longest_title_length + 1);

        println!(" # | √ | Date due   | Title");
        println!("---|---|------------|{}", title_divider);

        for (i, val) in self.store.iter()
            .filter(filter)
            .enumerate() {
            println!(" {} {}", i, val);
        }
    }
}
