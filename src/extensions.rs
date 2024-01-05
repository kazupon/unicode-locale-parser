use crate::errors::ParserError;
use crate::lang::UnicodeLanguageIdentifier;
use std::fmt::{self, Write};
use std::iter::Peekable;

#[derive(Debug)]
pub struct UnicodeLocaleExtensions {
    pub keyword: Vec<String>,
    pub attribute: Vec<String>,
}

#[derive(Debug)]
pub struct TransformedExtensions {
    pub tlang: Option<UnicodeLanguageIdentifier>,
    pub tfield: Vec<String>,
}

#[derive(Debug)]
pub struct PuExtensions {
    pub value: String,
}

#[derive(Debug)]
pub struct OtherExtensions {
    pub value: String,
}

#[warn(missing_docs)]
#[derive(Debug, PartialEq)]
pub enum ExtensionKind {
    UnicodeLocale,
    Transformed,
    Pu,
    Other(char),
}

impl ExtensionKind {
    pub fn from_byte(key: u8) -> Result<Self, ParserError> {
        let key = key.to_ascii_lowercase();
        match key {
            b'u' => Ok(ExtensionKind::UnicodeLocale),
            b't' => Ok(ExtensionKind::Transformed),
            b'x' => Ok(ExtensionKind::Pu),
            other if other.is_ascii_alphanumeric() => Ok(ExtensionKind::Other(char::from(other))),
            _ => Err(ParserError::InvalidExtension),
        }
    }
}

impl fmt::Display for ExtensionKind {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let c = match self {
            ExtensionKind::UnicodeLocale => 'u',
            ExtensionKind::Transformed => 't',
            ExtensionKind::Pu => 'x',
            ExtensionKind::Other(c) => *c,
        };
        f.write_char(c)
    }
}

#[derive(Debug)]
pub struct Extensions {
    pub unicode_locale: Option<UnicodeLocaleExtensions>,
    pub transformed: Option<TransformedExtensions>,
    pub pu: Option<PuExtensions>,
    pub other: Option<OtherExtensions>,
}

pub fn parse_unicode_extensions(chunk: &str) -> Result<Extensions, ParserError> {
    // check empty
    if chunk.is_empty() {
        return Err(ParserError::Missing); // TODO:
    }

    let mut iter = chunk.split(|c| c == '-' || c == '_').peekable();
    parse_unicode_extensions_from_iter(&mut iter)
}

pub fn parse_unicode_extensions_from_iter<'a>(
    iter: &mut Peekable<impl Iterator<Item = &'a str>>,
) -> Result<Extensions, ParserError> {
    Ok(Extensions {
        unicode_locale: None,
        transformed: None,
        pu: None,
        other: None,
    })
}

/**
 * Tests
 */

#[test]
fn success_extension_kind_from_byte() {
    assert_eq!(
        ExtensionKind::UnicodeLocale,
        ExtensionKind::from_byte(b'u').unwrap()
    );
    assert_eq!(
        ExtensionKind::Transformed,
        ExtensionKind::from_byte(b't').unwrap()
    );
    assert_eq!(
        ExtensionKind::Transformed,
        ExtensionKind::from_byte(b'T').unwrap()
    );
    assert_eq!(ExtensionKind::Pu, ExtensionKind::from_byte(b'x').unwrap());
    assert_eq!(
        ExtensionKind::Other('a'),
        ExtensionKind::from_byte(b'a').unwrap()
    );
    assert_eq!(
        ExtensionKind::Other('1'),
        ExtensionKind::from_byte(b'1').unwrap()
    );
}

#[test]
fn fail_extension_kind_from_byte() {
    assert_eq!(
        ParserError::InvalidExtension,
        ExtensionKind::from_byte(b'!').unwrap_err()
    );
    assert_eq!(
        ParserError::InvalidExtension,
        ExtensionKind::from_byte(b' ').unwrap_err()
    );
}

#[test]
fn extention_kind_display() {
    assert_eq!("u", format!("{}", ExtensionKind::UnicodeLocale));
    assert_eq!("t", format!("{}", ExtensionKind::Transformed));
    assert_eq!("x", format!("{}", ExtensionKind::Pu));
    assert_eq!("a", format!("{}", ExtensionKind::Other('a')));
}
