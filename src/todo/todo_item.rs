use std::fmt;
use chrono::{NaiveDate, format::ParseError, ParseResult};

#[derive(Debug)]
pub struct TodoItem {
    title: String,
    due_date: NaiveDate,
    pub complete: bool,
}

impl TodoItem {
    pub fn new(title: String, due_date: String) -> TodoItem {
        let due_date = match NaiveDate::parse_from_str(&due_date, "%Y-%m-%d") {
            Ok(result) => result,
            Err(err) => {
                println!("Due date was given in an invalid format: {due_date}");
                println!("Pattern must match: YYYY-MM-DD");
                panic!("{:#?}", err);
            }
        };

        TodoItem {
            title,
            due_date,
            complete: false,
        }
    }

    pub fn mark_as_done(&mut self) {
        self.complete = true;
    }
}

impl fmt::Display for TodoItem {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> std::fmt::Result {
        let is_done_indicator = match self.complete {
            true => String::from("[X]"),
            false => String::from("[ ]")
        };

        write!(
            f,
            "{is_done_indicator} {title}\nDue: {date}\n",
            title = self.title,
            date = self.due_date
        )
    }
}