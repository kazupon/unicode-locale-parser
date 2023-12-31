use std::error::Error;
use std::fmt::{Display, Formatter, Result};

// TODO: Should implement PartialEq and Eq?
#[derive(Debug)]
pub enum ParserError {
    // TODO: Add variants
}

impl Error for ParserError {}

impl Display for ParserError {
    fn fmt(&self, f: &mut Formatter) -> Result {
        match *self {
            // TODO:
            // ParserError::XXX => write!(f, "ParserError"),
        }
    }
}
