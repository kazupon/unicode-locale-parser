mod other;
mod pu;
mod transformed;
mod unicode_locale;

pub use other::OtherExtensions;
pub use pu::{pu_extensions, PuExtensions};
pub use transformed::TransformedExtensions;
pub use unicode_locale::UnicodeLocaleExtensions;

use crate::errors::ParserError;
use crate::utils::split_str;

use std::fmt::{self, Write};
use std::iter::Peekable;

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
    pub unicode_locale: Option<Vec<UnicodeLocaleExtensions>>,
    pub transformed: Option<Vec<TransformedExtensions>>,
    pub other: Option<Vec<OtherExtensions>>,
    pub pu: Option<PuExtensions>,
}

pub fn parse_unicode_extensions(chunk: &str) -> Result<Extensions, ParserError> {
    // check empty
    if chunk.is_empty() {
        return Err(ParserError::Missing); // TODO:
    }

    let mut iter = split_str(chunk).peekable();
    parse_unicode_extensions_from_iter(&mut iter)
}

pub fn parse_unicode_extensions_from_iter<'a>(
    iter: &mut Peekable<impl Iterator<Item = &'a str>>,
) -> Result<Extensions, ParserError> {
    let mut unicode_locale = vec![];
    let mut transformed = vec![];
    let mut other = vec![];
    let mut pu = None;

    let mut chunk = iter.next();
    while let Some(subtag) = chunk {
        match subtag
            .as_bytes()
            .first()
            .map(|c| ExtensionKind::from_byte(*c))
        {
            Some(Ok(ExtensionKind::UnicodeLocale)) => {
                unimplemented!("TODO: unicode locale extensions")
            }
            Some(Ok(ExtensionKind::Transformed)) => {
                unimplemented!("TODO: transformed extensions")
            }
            Some(Ok(ExtensionKind::Pu)) => {
                if pu.is_some() {
                    // TODO:
                    unimplemented!("TODO: should be throw error")
                }
                pu = Some(pu_extensions(iter)?);
            }
            Some(Ok(ExtensionKind::Other(c))) => {
                unimplemented!("TODO: other extensions")
            }
            None => {}
            _ => unreachable!(),
        }

        chunk = iter.next();
    }

    // normalize unicode locale extensions
    let unicode_locale = if unicode_locale.is_empty() {
        None
    } else {
        Some(unicode_locale)
    };

    // normalize transformed extensions
    let transformed = if transformed.is_empty() {
        None
    } else {
        Some(transformed)
    };

    // normalize other extensions
    let other = if other.is_empty() { None } else { Some(other) };

    Ok(Extensions {
        unicode_locale,
        transformed,
        pu,
        other,
    })
}

/**
 * Tests
 */

#[test]
fn success_parse_unicode_extensions() {
    let extensions = parse_unicode_extensions("x-foo-123").unwrap();
    assert_eq!("x-foo-123", format!("{}", extensions.pu.unwrap()));
}

#[test]
fn fail_parse_unicode_extensions() {
    // missing locale
    assert_eq!(
        ParserError::Missing,
        parse_unicode_extensions("").unwrap_err()
    );
}

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

/**
 * Unit tests
 */

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
