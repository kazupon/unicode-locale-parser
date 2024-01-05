use std::error::Error;
use std::fmt::{Display, Formatter, Result};

#[warn(missing_docs)]
#[derive(Debug, PartialEq)]
pub enum ParserError {
    MissingLanguage,
    InvalidLanguage,
    InvalidSubtag,
    MissingLocale,
    Unexpected,
}

impl Error for ParserError {}

impl Display for ParserError {
    fn fmt(&self, f: &mut Formatter) -> Result {
        let value = match self {
            ParserError::MissingLanguage => "Missing language identifier",
            ParserError::InvalidLanguage => "Invalid language identifier",
            ParserError::InvalidSubtag => "Invalid subtag",
            ParserError::MissingLocale => "Missing locale identifier",
            ParserError::Unexpected => "Unexpected error",
        };
        f.write_str(value)
    }
}
