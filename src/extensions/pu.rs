use crate::errors::ParserError;
use crate::utils::split_str;

use std::fmt::{self, Write};
use std::iter::Peekable;

#[derive(Debug)]
pub struct PuExtensions {
    pub values: Vec<String>,
}

impl fmt::Display for PuExtensions {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("x")?;
        for value in &self.values {
            f.write_char('-')?;
            f.write_str(value)?;
        }
        Ok(())
    }
}

pub fn pu_extensions<'a>(
    iter: &mut Peekable<impl Iterator<Item = &'a str>>,
) -> Result<PuExtensions, ParserError> {
    // pu_extensions
    // https://www.unicode.org/reports/tr35/tr35-71/tr35.html#pu_extensions
    let mut values = vec![];

    for subtag in iter {
        values.push(String::from(parse_value(subtag)?));
    }

    Ok(PuExtensions { values })
}

fn parse_value(subtag: &str) -> Result<&str, ParserError> {
    if subtag.is_empty()
        || subtag.len() > 8
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

#[test]
fn success_pu_extensions() {
    // full case
    let mut iter = split_str("abc-123").peekable();
    assert_eq!(vec!["abc", "123"], pu_extensions(&mut iter).unwrap().values);

    // Display trait implementation
    let mut iter = split_str("abc-123").peekable();
    assert_eq!(
        "x-abc-123",
        format!("{}", pu_extensions(&mut iter).unwrap())
    );
}

#[test]
fn fail_pu_extensions() {
    // invalid subtag
    let mut iter = split_str("abc-123456789").peekable();
    assert_eq!(
        ParserError::InvalidSubtag,
        pu_extensions(&mut iter).unwrap_err()
    );
}
