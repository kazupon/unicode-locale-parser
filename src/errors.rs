use std::error::Error;
use std::fmt::{Display, Formatter, Result};

#[derive(Debug, PartialEq)]
pub enum ParserError {
    MissingLangugage,
    // TODO: Add variants
}

impl Error for ParserError {}

impl Display for ParserError {
    fn fmt(&self, f: &mut Formatter) -> Result {
        let value = match self {
            ParserError::MissingLangugage => "Missing language identifier",
        };
        f.write_str(value)
    }
}
