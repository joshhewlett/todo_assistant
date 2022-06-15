use std::fmt;
use std::slice::Iter;

enum MenuAction {
    List,
    Create,
    Delete,
    History,
    ListAll,
}

struct MenuItem {
    action: MenuAction,
    name: &'static str,
    selection: u8,
}

impl MenuItem {
    pub fn parse_user_selection(input: &str) -> Result<&MenuItem, &str> {
        let input: u8 = input.trim().parse().expect("Please enter a number.");

        match MenuItem::iterator()
            .find(|menu_item| menu_item.selection == input) {
            Some(item) => Ok(item),
            None => Err("Please select a valid action.")
        }
    }

    pub fn iterator() -> Iter<'static, MenuItem> {
        static MENU_ITEMS: [MenuItem; 5] = [
            MenuItem {
                action: MenuAction::List,
                name: &"List",
                selection: 0,
            },
            MenuItem {
                action: MenuAction::Create,
                name: &"Create",
                selection: 1,
            },
            MenuItem {
                action: MenuAction::Delete,
                name: &"Delete",
                selection: 2,
            },
            MenuItem {
                action: MenuAction::History,
                name: &"History",
                selection: 3,
            },
            MenuItem {
                action: MenuAction::ListAll,
                name: &"ListAll",
                selection: 4,
            }];
        MENU_ITEMS.iter()
    }
}

impl fmt::Display for MenuItem {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {

        write!(f, "{}: {}", self.selection, self.name)
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
    MenuItem::iterator().for_each(|action| println!("{}", action));
}