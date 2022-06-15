use std::fmt;
use std::slice::Iter;

#[derive(Debug)]
enum MenuAction {
    List { name: String, item_number: u8 },
    Create { name: String, item_number: u8 },
    Delete { name: String, item_number: u8 },
    History { name: String, item_number: u8 },
    ListAll { name: String, item_number: u8 },
}

/*
============= There is no generic field all MenuActions share... That's why it's not working
 */

impl MenuAction {
    pub fn parse_user_selection(input: &str) -> Result<&MenuAction, &str> {
        let input: u8 = input.trim().parse().expect("Please enter a number.");

        // MenuAction::iterator()
        //     .for_each(|action| println!("{}", action.item_number));

        Ok(&MenuAction::List { name: String::from("name"), item_number: 0 })
            // .find(|action| action.item_number == input)
            // .ok_or_else(Err("Please select a valid action."))
    }

    pub fn iterator() -> Iter<'static, MenuAction> {
        static MENU_ACTIONS: [MenuAction; 5] = [
            MenuAction::List {
                name: String::from("List"),
                item_number: 0,
            },
            MenuAction::Create {
                name: String::from("Create"),
                item_number: 1,
            },
            MenuAction::Delete {
                name: String::from("Delete"),
                item_number: 2,
            },
            MenuAction::History {
                name: String::from("History"),
                item_number: 3,
            },
            MenuAction::ListAll {
                name: String::from("ListAll"),
                item_number: 4,
            }];
        MENU_ACTIONS.iter()
    }
}

impl fmt::Display for MenuAction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}: {}", self.item_number, self.name)
    }
}

pub fn run() {

    // Show user menu
    // Ask user for input
    // Env var for default open behavior?
    // Execute task
    // Write data to file for each update
    get_menu_action();
}

fn get_menu_action() -> MenuAction {
    print_menu();
}

fn print_menu() {
    println!("Please select an action:");
    MenuAction::iterator().for_each(|action| println!("{}", action));
}