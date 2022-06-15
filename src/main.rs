extern crate core;

mod todo;

use todo::{todo_store::TodoStore, todo_item::TodoItem};

fn main() {

    // Init data from file
    let mut todo_store = TodoStore::new();
    // todo_store.mark_as_done(0);
    //
    // todo_store.list_all_todos();
    //
    // todo_store.create_new_todo(
    //     TodoItem::new(
    //         String::from("Todo Number 4"),
    //         String::from("2022-02-01")));
    //
    // println!("========");
    // todo_store.list_all_todos();

    todo_assistant::run();
}

