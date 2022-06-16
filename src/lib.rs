mod error;

use std::{fmt, io};
use std::error::Error;
use std::fmt::Pointer;
use std::num::ParseIntError;
use std::slice::Iter;

use error::todo_error::TodoError;


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
    pub fn parse_user_selection(input: &str) -> Result<&MenuItem, TodoError> {
        let input: u8 = input.trim().parse::<u8>()
            .map_err(|_| TodoError::new_from_msg(String::from("Selection must be a number.")))?;

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

    print_menu();

    //
    let mut user_selection = String::new();
    io::stdin().read_line(&mut user_selection)
        // .map_err(|_| Err(String::from("Failed to read line.")))?;
        .map_err(|_| TodoError::new_from_msg(String::from("Failed to read line.")))?;

    let menu_item_selection = MenuItem::parse_user_selection(user_selection.as_str())?;
    match menu_item_selection.action {
        MenuAction::List => {}
        MenuAction::Create => {}
        MenuAction::Delete => {}
        MenuAction::History => {}
        MenuAction::ListAll => {}
        MenuAction::Quit => {}
    }

    println!("Selected: {}", menu_item_selection.name);

    Ok(())
}

// fn get_menu_action() -> MenuAction {
//     print_menu();
// }

fn print_menu() {
    println!("Please select an action:");
    MenuItem::iterator().for_each(|action| println!("{}", action));
}