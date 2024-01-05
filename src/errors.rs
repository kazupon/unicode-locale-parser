use std::error::Error;
use std::fmt::{Display, Formatter, Result};

#[warn(missing_docs)]
#[derive(Debug, PartialEq)]
pub enum ParserError {
    MissingLanguage,
    InvalidLanguage,
    InvalidSubtag,
    MissingLocale,
    InvalidExtension,
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
            ParserError::InvalidExtension => "Invalid extension",
            ParserError::Unexpected => "Unexpected error",
        };
        f.write_str(value)
    }
}
