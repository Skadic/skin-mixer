use std::{error::Error, fmt::Display};

#[derive(Debug)]
pub struct InvalidElemTypeError{
    name: String
}

impl InvalidElemTypeError {
    pub fn new(name: &str) -> Self {
        Self { name: name.to_owned() }
    }
}

impl Display for InvalidElemTypeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Invalid element type name: {}", self.name)
    }
}


impl Error for InvalidElemTypeError {}