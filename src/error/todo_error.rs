use std::error::Error;
use std::fmt;

#[derive(Debug)]
pub struct TodoError {
    message: String,
    source: Option<Box<dyn Error>>
}

impl TodoError {

    pub fn new(message: String, source: Box<(dyn Error)>) -> TodoError {

        TodoError {
            message,
            source: Some(source)
        }
    }

    pub fn new_from_msg(message: String) -> TodoError {

        TodoError {
            message,
            source: Option::None
        }
    }

    pub fn root(&self) -> &Option<Box<dyn Error>> {

        &self.source
    }
}

impl Error for TodoError {}

impl fmt::Display for TodoError {

    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}