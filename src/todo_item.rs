use std::fmt;
use chrono::{NaiveDate};
use regex::Regex;
use serde::{Serialize, Deserialize};
use crate::todo_error::TodoError;

#[derive(Debug, Serialize, Deserialize)]
pub struct TodoItemSerializable {
    pub id: usize,
    pub title: String,
    pub due_date: String,
    pub complete: bool,
}

// TODO: Maybe add date_completed
pub struct TodoItem {
    pub id: usize,
    pub title: String,
    pub due_date: NaiveDate,
    pub complete: bool,
}

impl TodoItem {
    pub fn new(args: String, id: usize) -> Result<TodoItem, TodoError> {
        let regex_pattern = r"^(\d{4}-[0-1]\d-[0-3]\d)\s([A-Za-z0-9-_?.<> ]{1,50})$";
        let regex = Regex::new(regex_pattern).unwrap();

        if !regex.is_match(&args.trim()) {
            return Err(TodoError::new_from_msg(String::from("Invalid format for new Todo item.")));
        }

        let captures = regex.captures(&args.trim()).unwrap();
        let due_date = &captures[1];
        let title = String::from(&captures[2]);

        let due_date = NaiveDate::parse_from_str(due_date, "%Y-%m-%d").unwrap();

        Ok(TodoItem {
            id,
            title,
            due_date,
            complete: false,
        })
    }

    pub fn deserialize(dto: TodoItemSerializable) -> Result<TodoItem, TodoError> {
        let due_date = NaiveDate::parse_from_str(&dto.due_date, "%Y-%m-%d").unwrap();

        Ok(TodoItem {
            id: dto.id,
            title: dto.title,
            due_date,
            complete: dto.complete,
        })
    }

    pub fn mark_as_done(&mut self) {
        self.complete = true;
    }

    pub fn to_serializable(&self) -> TodoItemSerializable {
        TodoItemSerializable {
            id: self.id,
            title: self.title.clone(),
            due_date: self.due_date.to_string(),
            complete: self.complete,
        }
    }
}

impl fmt::Display for TodoItem {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let is_done_indicator = match self.complete {
            true => String::from("X"),
            false => String::from(" ")
        };

        write!(f, " {} | {} | {} | {}", self.id, is_done_indicator, self.due_date, self.title)
    }
}

#[cfg(test)]
mod todoitem_new_tests {
    use super::*;

    #[test]
    fn greenpath() {
        let user_input = "2021-01-01 First Todo";
        let result = TodoItem::new(String::from(user_input), 0).unwrap();

        let expected_date = NaiveDate::parse_from_str("2021-01-01", "%Y-%m-%d").unwrap();
        assert_eq!(expected_date, result.due_date);
        assert_eq!("First Todo", result.title);
        assert!(!result.complete);
    }

    #[test]
    fn greenpath_whitepsace_around_input() {
        let user_input = "    2022-01-01 First Todo    ";
        let result = TodoItem::new(String::from(user_input), 0).unwrap();

        let expected_date = NaiveDate::parse_from_str("2022-01-01", "%Y-%m-%d").unwrap();
        assert_eq!(expected_date, result.due_date);
        assert_eq!("First Todo", result.title);
        assert!(!result.complete);
    }

    #[test]
    fn bad_input() {
        let user_input = "BAD INPUT";
        let error = TodoItem::new(String::from(user_input), 0).err().unwrap();

        assert_eq!("Invalid format for new Todo item.", &error.message);
    }

    #[test]
    fn bad_input_too_long() {
        let user_input = String::from("2022-01-01 Todo Item") + String::from("0").repeat(50).as_str();
        let error = TodoItem::new(String::from(user_input), 0).err().unwrap();

        assert_eq!("Invalid format for new Todo item.", &error.message);
    }
}
