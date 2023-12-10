use std::error::Error;
use std::fmt;

#[derive(Debug)]
pub enum CustomError {
    NoEventFound,
    EventAlreadyStarted,
    CannotRenderInsights(String),
    EventNotStarted,
}

impl fmt::Display for CustomError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            Self::NoEventFound => write!(f, "No event found"),
            Self::EventAlreadyStarted => write!(
                f,
                "Event already started. Cannot write insights on started or finished event."
            ),
            Self::CannotRenderInsights(ref error) => {
                write!(f, "Cannot get insights for the event: {}", error)
            }
            Self::EventNotStarted => write!(f, "Event not started yet"),
        }
    }
}

impl Error for CustomError {}
