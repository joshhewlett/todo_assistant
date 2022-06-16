use std::error::Error;
use std::fmt;

#[derive(Debug)]
pub struct TodoError {
    message: String,
    source: Option<Box<dyn Error + 'static>>
}

impl TodoError {

    pub fn new(message: String, source: Option<Box<(dyn Error + 'static)>>) -> TodoError {

        TodoError {
            message,
            source
            // source: match source {
            //     None => None,
            //     Some(err) => Some(Box::new(err))
            // }
        }
    }

    pub fn new_from_msg(message: String) -> TodoError {

        TodoError {
            message,
            source: Option::None
        }
    }
}

impl Error for TodoError {

    fn source(&self) -> Option<&(dyn Error + 'static)> {

        match &self.source {
            None => None,
            Some(err) => Some(err.source().unwrap())
        }
    }
}

impl fmt::Display for TodoError {

    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}