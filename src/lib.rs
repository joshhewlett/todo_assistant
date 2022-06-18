use std::{fmt, io, process};
use std::error::Error;
use std::fmt::Pointer;
use std::num::ParseIntError;
use std::slice::Iter;

mod error;

use error::todo_error::TodoError;
// use crate::error::todo_error::TodoError;

mod todo;

use todo::todo_store::TodoStore;

enum MenuAction {
    List,
    Create,
    Delete,
    History,
    ListAll,
    Quit,
}

struct MenuItem {
    action: MenuAction,
    name: &'static str,
    selection: u8,
}

impl MenuItem {
    pub fn parse_user_selection(input: &String) -> Result<&'static MenuItem, TodoError> {
        let input: u8 = input.trim().parse::<u8>()
            .map_err(|err| TodoError::new(
                String::from("Selection must be a number."),
                Box::new(err)))?;

        MenuItem::iterator()
            .find(|menu_item| menu_item.selection == input)
            .ok_or(TodoError::new_from_msg(String::from("A valid menu action must be selected.")))
    }

    pub fn iterator() -> Iter<'static, MenuItem> {
        static MENU_ITEMS: [MenuItem; 6] = [
            MenuItem {
                action: MenuAction::List,
                name: &"List incomplete items",
                selection: 0,
            },
            MenuItem {
                action: MenuAction::Create,
                name: &"Create new item",
                selection: 1,
            },
            MenuItem {
                action: MenuAction::Delete,
                name: &"Delete item",
                selection: 2,
            },
            MenuItem {
                action: MenuAction::History,
                name: &"Show history",
                selection: 3,
            },
            MenuItem {
                action: MenuAction::ListAll,
                name: &"List all items",
                selection: 4,
            },
            MenuItem {
                action: MenuAction::Quit,
                name: &"Quit",
                selection: 5,
            }];
        MENU_ITEMS.iter()
    }
}

impl fmt::Display for MenuItem {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} - {}", self.selection, self.name)
    }
}

pub fn run() -> Result<(), Box<TodoError>> {

    // Show user menu
    // Ask user for input
    // Env var for default open behavior?
    // Execute task
    // Write data to file for each update
    // get_menu_action();

    // TODO: Init data from file
    let mut store = TodoStore::new()?;

    loop {
        print_menu();

        let menu_item_selection = get_menu_action()?;
        // println!("Selected: {}", menu_item_selection.name);

        match menu_item_selection.action {
            MenuAction::List => { store.list_incomplete_todos(); }
            MenuAction::Create => { store.create_new_todo()?; }
            MenuAction::Delete => {}
            MenuAction::History => { store.list_history(); }
            MenuAction::ListAll => { store.list_all_todos(); }
            MenuAction::Quit => {
                // TODO: Save state
                println!("Goodbye.");
                process::exit(0);
            }
        }

        println!();
    }
}

fn get_menu_action() -> Result<&'static MenuItem, TodoError> {
    let mut user_selection = String::new();
    io::stdin().read_line(&mut user_selection)
        .map_err(|err| TodoError::new(
            String::from("Failed to read line."),
            Box::new(err)))?;

    MenuItem::parse_user_selection(&user_selection)
}

fn print_menu() {
    println!("Please select an action:");
    MenuItem::iterator().for_each(|action| println!("{}", action));
}