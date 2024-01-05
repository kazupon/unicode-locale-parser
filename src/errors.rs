use std::error::Error;
use std::fmt::{Display, Formatter, Result};

#[warn(missing_docs)]
#[derive(Debug, PartialEq)]
pub enum ParserError {
    Missing,
    InvalidLanguage,
    InvalidSubtag,
    InvalidExtension,
    Unexpected,
}

impl Error for ParserError {}

impl Display for ParserError {
    fn fmt(&self, f: &mut Formatter) -> Result {
        let value = match self {
            ParserError::Missing => "Missing identifier",
            ParserError::InvalidLanguage => "Invalid language identifier",
            ParserError::InvalidSubtag => "Invalid subtag",
            ParserError::InvalidExtension => "Invalid extension",
            ParserError::Unexpected => "Unexpected error",
        };
        f.write_str(value)
    }
}
