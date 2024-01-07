use crate::constants::SEP;
use crate::errors::ParserError;

use std::fmt::{self, Write};
use std::iter::Peekable;

#[derive(Debug, PartialEq)]
pub struct OtherExtensions {
    pub values: Vec<String>,
    pub extension: char,
}

impl fmt::Display for OtherExtensions {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_char(self.extension)?;
        for value in &self.values {
            f.write_char(SEP)?;
            f.write_str(value)?;
        }
        Ok(())
    }
}

pub fn parse_other_extensions<'a>(
    iter: &mut Peekable<impl Iterator<Item = &'a str>>,
    extension: char,
) -> Result<OtherExtensions, ParserError> {
    // other_extensions
    // https://www.unicode.org/reports/tr35/tr35-71/tr35.html#other_extensions
    let mut values = vec![];

    while let Some(subtag) = iter.peek() {
        if subtag.len() == 1 {
            break;
        } else {
            values.push(String::from(parse_value(subtag)?));
            iter.next();
        }
    }

    Ok(OtherExtensions { values, extension })
}

fn parse_value(subtag: &str) -> Result<&str, ParserError> {
    if !(2..=8).contains(&subtag.len())
        || !subtag.as_bytes().iter().all(|c| c.is_ascii_alphanumeric())
    {
        Err(ParserError::InvalidSubtag)
    } else {
        Ok(subtag)
    }
}

/**
 * Unit tests
 */

#[allow(unused_imports)] // for unit tests
use crate::shared::split_str;

#[test]
fn success_other_extensions() {
    // full case
    let mut iter = split_str("abc-123").peekable();
    assert_eq!(
        vec!["abc", "123"],
        parse_other_extensions(&mut iter, 'a').unwrap().values
    );

    // Display trait implementation
    let mut iter = split_str("abc-123").peekable();
    assert_eq!(
        "b-abc-123",
        format!("{}", parse_other_extensions(&mut iter, 'b').unwrap())
    );
}

#[test]
fn fail_pu_extensions() {
    // invalid subtag
    let mut iter = split_str("abc-123456789").peekable();
    assert_eq!(
        ParserError::InvalidSubtag,
        parse_other_extensions(&mut iter, '1').unwrap_err()
    );
}
