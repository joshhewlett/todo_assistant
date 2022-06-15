use std::fmt;

#[derive(Debug)]
enum MenuAction {
    List,
    Create,
    Delete,
    History,
    ListAll,
}

/*
============= There is no generic field all MenuActions share... That's why it's not working
 */

impl MenuAction {
    pub fn parse_user_selection(input: &str) -> Result<MenuAction, &str> {
        let input: u8 = input.trim().parse().expect("Please enter a number.");

        match input {
            0 => Ok(MenuAction::List),
            1 => Ok(MenuAction::Create),
            2 => Ok(MenuAction::Delete),
            3 => Ok(MenuAction::History),
            4 => Ok(MenuAction::ListAll),
            _ => Err("Please select a valid action.")
        }
    }
}

impl fmt::Display for MenuAction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {

        let val = match self {
            MenuAction::List => (0, "List"),
            MenuAction::Create => (1, "Create"),
            MenuAction::Delete => (2, "Delete"),
            MenuAction::History => (3, "History"),
            MenuAction::ListAll => (4, "ListAll")
        };

        write!(f, "{}: {}", val.0, val.1)
    }
}

pub fn run() {

    // Show user menu
    // Ask user for input
    // Env var for default open behavior?
    // Execute task
    // Write data to file for each update
    // get_menu_action();
    print_menu();
}

// fn get_menu_action() -> MenuAction {
//     print_menu();
// }

fn print_menu() {
    println!("Please select an action:");
    println!("{}", MenuAction::List);
    println!("{}", MenuAction::Create);
    println!("{}", MenuAction::Delete);
    println!("{}", MenuAction::History);
    println!("{}", MenuAction::ListAll);
    // MenuAction::iterator().for_each(|action| println!("{}", action));
}