use std::fmt;
use chrono::{NaiveDate, format::ParseError, ParseResult};
use regex::Regex;
use crate::error::todo_error::TodoError;

// TODO: Maybe add date_completed
#[derive(Debug)]
pub struct TodoItem {
    pub title: String,
    pub due_date: NaiveDate,
    pub complete: bool,
}

impl TodoItem {
    pub fn new(args: String) -> Result<TodoItem, TodoError> {
        let regex_pattern = r"^(\d{4}-[0-1]\d-[0-3]\d)\s([A-Za-z0-9-_?. ]{1,50})$";
        let regex = Regex::new(regex_pattern).unwrap();

        if !regex.is_match(&args.trim()) {
            return Err(TodoError::new_from_msg(String::from("Invalid format for new Todo item.")));
        }

        let captures = regex.captures(&args.trim()).unwrap();
        let due_date = &captures[1];
        let title = String::from(&captures[2]);

        let due_date = NaiveDate::parse_from_str(due_date, "%Y-%m-%d").unwrap();

        Ok(TodoItem {
            title,
            due_date,
            complete: false,
        })
    }

    pub fn mark_as_done(&mut self) {
        self.complete = true;
    }
}

impl fmt::Display for TodoItem {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let is_done_indicator = match self.complete {
            true => String::from("X"),
            false => String::from(" ")
        };

        write!(f, "| {} | {} | {}", is_done_indicator, self.due_date, self.title)
    }
}


#[cfg(test)]
mod todoitem_new_tests {
    use super::*;

    #[test]
    fn greenpath() {
        let user_input = "2021-01-01 First Todo";
        let result = TodoItem::new(String::from(user_input)).unwrap();

        let expected_date = NaiveDate::parse_from_str("2021-01-01", "%Y-%m-%d").unwrap();
        assert_eq!(expected_date, result.due_date);
        assert_eq!("First Todo", result.title);
        assert!(!result.complete);
    }

    #[test]
    fn greenpath_whitepsace_around_input() {
        let user_input = "    2022-01-01 First Todo    ";
        let result = TodoItem::new(String::from(user_input)).unwrap();

        let expected_date = NaiveDate::parse_from_str("2022-01-01", "%Y-%m-%d").unwrap();
        assert_eq!(expected_date, result.due_date);
        assert_eq!("First Todo", result.title);
        assert!(!result.complete);
    }

    #[test]
    fn bad_input() {
        let user_input = "BAD INPUT";
        let error = TodoItem::new(String::from(user_input)).err().unwrap();

        assert_eq!("Invalid format for new Todo item.", &error.message);
    }

    #[test]
    fn bad_input_too_long() {
        let user_input = String::from("2022-01-01 Todo Item") + String::from("0").repeat(50).as_str();
        let error = TodoItem::new(String::from(user_input)).err().unwrap();

        assert_eq!("Invalid format for new Todo item.", &error.message);
    }
}
