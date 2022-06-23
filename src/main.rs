extern crate core;

mod error;

fn main() {

    // TODO: These loops are probably not the best way to do this...
    loop {
        if let Err(e) = todo_assistant::run() {
            eprintln!("Error: {}", e);
            if e.root().is_some() {
                eprintln!("Root cause: {}", e.root().as_ref().unwrap());
            }
        }
    }
}