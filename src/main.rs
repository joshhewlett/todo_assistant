extern crate core;

use todo_assistant::todo::TodoStore;

const PERSISTENCE_STORE_FILENAME: &str = "todo_store_data.json";

fn main() {

    let mut store = TodoStore::new_from_persistence(PERSISTENCE_STORE_FILENAME).unwrap();

    loop {
        if let Err(e) = todo_assistant::run(&mut store) {
            eprintln!("Error: {}", e);
            if e.root().is_some() {
                eprintln!("Root cause: {}", e.root().as_ref().unwrap());
            }
        }
    }
}
