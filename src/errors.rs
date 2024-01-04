use std::error::Error;
use std::fmt::{Display, Formatter, Result};

#[derive(Debug, PartialEq)]
pub enum ParserError {
    MissingLanguage,
    InvalidLanguage,
    InvalidSubtag,
    Unexpected,
}

impl Error for ParserError {}

impl Display for ParserError {
    fn fmt(&self, f: &mut Formatter) -> Result {
        let value = match self {
            ParserError::MissingLanguage => "Missing language identifier",
            ParserError::InvalidLanguage => "Invalid language identifier",
            ParserError::InvalidSubtag => "Invalid language subtag",
            ParserError::Unexpected => "Unexpected error",
        };
        f.write_str(value)
    }
}
