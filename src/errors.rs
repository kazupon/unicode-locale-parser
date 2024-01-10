use std::error::Error;
use std::fmt::{Display, Formatter, Result};

/// Enum representing the possible errors that can occur when parsing [Unicode UTS #35 Language and Locale Identifiers](https://unicode.org/reports/tr35/#Identifiers).
#[derive(Debug, PartialEq)]
pub enum ParserError {
  /// A missing identifier error.
  Missing,
  /// An invalid language identifier error.
  InvalidLanguage,
  /// An invalid subtag error.
  InvalidSubtag,
  /// An invalid unicode extensions error.
  InvalidExtension,
  /// An invalid unicode subdivision error.
  InvalidSubdivision,
  /// An unexpected error.
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
      ParserError::InvalidSubdivision => "Invalid subdivision",
      ParserError::Unexpected => "Unexpected error",
    };
    f.write_str(value)
  }
}
