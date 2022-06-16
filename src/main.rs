extern crate core;

mod todo;
mod error;

use std::error::Error;
use todo::{todo_store::TodoStore, todo_item::TodoItem};

fn main() {

    // TODO: Init data from file

    loop {
        println!();
        if let Err(e) = todo_assistant::run() {
            eprintln!("Error: {}", e);
            if e.root().is_some() {
                eprintln!("Root cause: {}", e.root().as_ref().unwrap());
            }
        }
    }
}

