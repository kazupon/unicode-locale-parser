pub mod other;
pub mod pu;
pub mod transformed;
pub mod unicode_locale;

use other::{parse_other_extensions, OtherExtensions};
use pu::{parse_pu_extensions, PuExtensions};
use transformed::{parse_transformed_extensions, TransformedExtensions};
use unicode_locale::{parse_unicode_locale_extensions, UnicodeLocaleExtensions};

use crate::constants::SEP;
use crate::errors::ParserError;
use crate::shared::split_str;

use std::fmt::{self, Write};
use std::iter::Peekable;

#[derive(Debug, PartialEq)]
enum ExtensionKind {
    UnicodeLocale,
    Transformed,
    Pu,
    Other(char),
}

impl ExtensionKind {
    fn from_byte(key: u8) -> Result<Self, ParserError> {
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

impl fmt::Display for Extensions {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut messages = vec![];
        if let Some(unicode_locale) = &self.unicode_locale {
            for u in unicode_locale {
                messages.push(format!("{}", u));
            }
        }
        if let Some(transformed) = &self.transformed {
            for t in transformed {
                messages.push(format!("{}", t));
            }
        }
        if let Some(other) = &self.other {
            for o in other {
                messages.push(format!("{}", o));
            }
        }
        if let Some(pu) = &self.pu {
            messages.push(format!("{}", pu));
        }

        if !messages.is_empty() {
            f.write_str(&messages.join(&SEP.to_string()))?;
        }
        Ok(())
    }
}

pub fn parse_extensions(chunk: &str) -> Result<Extensions, ParserError> {
    // check empty
    if chunk.is_empty() {
        return Err(ParserError::Missing);
    }

    parse_extensions_from_iter(&mut split_str(chunk).peekable())
}

pub fn parse_extensions_from_iter<'a>(
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
                unicode_locale.push(parse_unicode_locale_extensions(iter)?);
            }
            Some(Ok(ExtensionKind::Transformed)) => {
                transformed.push(parse_transformed_extensions(iter)?);
            }
            Some(Ok(ExtensionKind::Pu)) => {
                if pu.is_some() {
                    return Err(ParserError::Unexpected);
                }
                pu = Some(parse_pu_extensions(iter)?);
            }
            Some(Ok(ExtensionKind::Other(c))) => {
                other.push(parse_other_extensions(iter, c)?);
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
fn success_parse_extensions() {
    // basic
    let extensions = parse_extensions(
        "U-attr1-kz-value2-t-en-Latn-US-macos-t1-value1-value2-a-vue-rust-x-foo-123",
    )
    .unwrap();
    let unicode_locale = extensions.unicode_locale.unwrap();
    assert_eq!(
        ["u-attr1-kz-value2"],
        unicode_locale
            .iter()
            .map(|u| format!("{}", u))
            .collect::<Vec<String>>()
            .as_slice()
    );
    let transformed = extensions.transformed.unwrap();
    assert_eq!(
        ["t-en-Latn-US-macos-t1-value1-value2"],
        transformed
            .iter()
            .map(|t| format!("{}", t))
            .collect::<Vec<String>>()
            .as_slice()
    );
    let other = extensions.other.unwrap();
    assert_eq!(
        ["a-vue-rust"],
        other
            .iter()
            .map(|o| format!("{}", o))
            .collect::<Vec<String>>()
            .as_slice()
    );
    let pu = extensions.pu.unwrap();
    assert_eq!("x-foo-123", format!("{}", pu));

    // Display trait implementation
    assert_eq!(
        "u-attr1-kz-value2-t-en-Latn-US-macos-t1-value1-value2-a-vue-rust-x-foo-123",
        format!(
            "{}",
            parse_extensions(
                "U-attr1-kz-value2-t-en-Latn-US-macos-t1-value1-value2-a-vue-rust-x-foo-123",
            )
            .unwrap()
        )
    );
}

#[test]
fn fail_parse_unicode_extensions() {
    // missing locale
    assert_eq!(ParserError::Missing, parse_extensions("").unwrap_err());
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
