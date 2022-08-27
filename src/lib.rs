use std::{fmt, io, process};
use std::io::Write;

pub mod err;
pub mod todo;

use err::TodoError;
use todo::{TodoItem, TodoStore};

// TODO: Fix deserialization error loop
enum MenuAction {
    ListIncompleteItems,
    CreateItem,
    MarkItemComplete,
    ListCompletedItems,
    ListAllItems,
    Quit,
}

struct MenuItem {
    action: MenuAction,
    title: &'static str,
    selection: char,
}

impl MenuItem {
    pub fn parse_user_selection(input: &String) -> Result<&'static MenuItem, TodoError> {
        let input: char = input.trim().parse::<char>().map_err(|err| {
            TodoError::new(
                String::from("Input must be a single character."),
                Box::new(err),
            )
        })?;

        MENU_ITER
            .iter()
            .find(|menu_item| menu_item.selection == input)
            .ok_or(TodoError::new_from_msg(String::from(
                "A valid menu action must be selected.",
            )))
    }
}

const MENU_ITER: [MenuItem; 6] = [
    LIST_INCOMPLETE_ITEMS,
    LIST_ALL_ITEMS,
    LIST_COMPLETED_ITEMS,
    CREATE_ITEM,
    COMPLETE_ITEM,
    QUIT,
];

const LIST_INCOMPLETE_ITEMS: MenuItem = MenuItem {
    action: MenuAction::ListIncompleteItems,
    title: &"List [i]ncomplete items",
    selection: 'i',
};
const LIST_ALL_ITEMS: MenuItem = MenuItem {
    action: MenuAction::ListAllItems,
    title: &"List [a]ll items",
    selection: 'a',
};
const LIST_COMPLETED_ITEMS: MenuItem = MenuItem {
    action: MenuAction::ListCompletedItems,
    title: &"List [h]istory",
    selection: 'h',
};
const CREATE_ITEM: MenuItem = MenuItem {
    action: MenuAction::CreateItem,
    title: &"Create [n]ew item",
    selection: 'n',
};
const COMPLETE_ITEM: MenuItem = MenuItem {
    action: MenuAction::MarkItemComplete,
    title: &"[C]omplete item",
    selection: 'c',
};
const QUIT: MenuItem = MenuItem {
    action: MenuAction::Quit,
    title: &"[Q]uit...",
    selection: 'q',
};

const MENU_COLUMN_WIDTH: usize = 30; // Must be large than longest MenuAction.title

impl fmt::Display for MenuItem {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} - {}", self.selection, self.title)
    }
}

pub fn run(store: &mut TodoStore) -> Result<(), Box<TodoError>> {
    // Show user menu
    // Ask user for input
    // Env var for default open behavior?
    // Execute task
    // Write data to file for each update
    // get_menu_action();

    print_menu();

    let menu_item_selection = get_menu_action()?;
    // println!("Selected: {}", menu_item_selection.name);

    match menu_item_selection.action {
        MenuAction::ListIncompleteItems => {
            todo::print_store("Incomplete items", &store.list_incomplete_todos());
        }
        MenuAction::ListAllItems => {
            todo::print_store("All items", &store.list_all_todos());
        }
        MenuAction::ListCompletedItems => {
            todo::print_store("Completed items", &store.list_history());
        }
        MenuAction::CreateItem => {
            store.create_new_todo()?;
        }
        MenuAction::MarkItemComplete => {
            todo::print_store("Incomplete items", &store.list_incomplete_todos());
            store.mark_as_done()?;
        }
        MenuAction::Quit => {
            // TODO: Save state
            println!("Goodbye.");
            process::exit(0);
        }
    }
    println!();

    Ok(())
}

fn get_menu_action() -> Result<&'static MenuItem, TodoError> {
    print!("> ");
    io::stdout().flush().unwrap();

    let mut user_selection = String::new();
    io::stdin()
        .read_line(&mut user_selection)
        .map_err(|err| TodoError::new(String::from("Failed to read line."), Box::new(err)))?;

    MenuItem::parse_user_selection(&user_selection)
}

fn print_menu() {
    println!("\nPlease select an action:");

    let column_page_size = (MENU_ITER.len() / 2) + (MENU_ITER.len() % 2);

    for i in 0..column_page_size {
        let buffer_length = MENU_COLUMN_WIDTH - format!("{}", MENU_ITER[i]).len();
        let buffer = String::from(" ").repeat(buffer_length);

        let column_one_title = format!("{}", MENU_ITER[i]) + &buffer;
        let column_two_title = MENU_ITER
            .get(i + column_page_size)
            .map(|item| format!("{}", item))
            .unwrap_or(String::from(""));

        println!("{}{}", column_one_title, column_two_title);
    }
}
