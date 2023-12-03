use std::error::Error;
use std::fmt;

#[derive(Debug)]
pub enum CustomError {
    NoEventFound,
}

impl fmt::Display for CustomError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            Self::NoEventFound => write!(f, "No event found"),
        }
    }
}

impl Error for CustomError {}