use std::error::Error;
use std::fmt::Display;
use std::io;
use std::num::ParseIntError;

#[derive(Debug)]
pub struct WfcError {
    message: String,
}

impl WfcError {
    pub fn new(message: String) -> Self {
        Self { message }
    }
}

impl Display for WfcError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl Error for WfcError {}

impl From<io::Error> for WfcError {
    fn from(value: io::Error) -> Self {
        Self {
            message: value.to_string(),
        }
    }
}

impl From<serde_yaml::Error> for WfcError {
    fn from(value: serde_yaml::Error) -> Self {
        Self {
            message: value.to_string(),
        }
    }
}

impl From<ParseIntError> for WfcError {
    fn from(value: ParseIntError) -> Self {
        Self {
            message: value.to_string(),
        }
    }
}
