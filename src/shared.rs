use std::error::Error;
use std::fmt::Display;
use std::io;
use std::num::ParseIntError;

#[derive(Debug)]
pub struct WFCError {
    message: String,
}

impl WFCError {
    pub fn new(message: String) -> Self {
        Self { message }
    }
}

impl Display for WFCError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl Error for WFCError {}

impl From<io::Error> for WFCError {
    fn from(value: io::Error) -> Self {
        Self {
            message: value.to_string(),
        }
    }
}

impl From<serde_yaml::Error> for WFCError {
    fn from(value: serde_yaml::Error) -> Self {
        Self {
            message: value.to_string(),
        }
    }
}

impl From<ParseIntError> for WFCError {
    fn from(value: ParseIntError) -> Self {
        Self {
            message: value.to_string(),
        }
    }
}
