use crate::errors::ParserError;

use std::fmt::{self};
use std::str;
use std::str::FromStr;

#[derive(Debug, PartialEq)]
pub struct UnicodeSubdivisionIdentifier {
    pub region: String,
    pub suffix: String,
}

impl fmt::Display for UnicodeSubdivisionIdentifier {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}{}", self.region, self.suffix)?;
        Ok(())
    }
}

impl FromStr for UnicodeSubdivisionIdentifier {
    type Err = ParserError;

    fn from_str(source: &str) -> Result<Self, Self::Err> {
        parse_unicode_subdivision_id(source)
    }
}

pub fn parse_unicode_subdivision_id(
    chunk: &str,
) -> Result<UnicodeSubdivisionIdentifier, ParserError> {
    // unicode_subdivision_id
    // https://unicode.org/reports/tr35/#unicode_subdivision_id

    let chunk = chunk.as_bytes();

    if chunk.is_empty() {
        return Err(ParserError::Missing);
    }

    let len = chunk.len();
    if !(2..=7).contains(&len) {
        return Err(ParserError::InvalidSubdivision);
    }

    let region_index = region_index(chunk)?;
    let region = match str::from_utf8(&chunk[0..region_index]) {
        Ok(s) => s,
        Err(_) => return Err(ParserError::Unexpected),
    };

    let suffix_len = len - region_index;
    if !(3..7).contains(&suffix_len)
        || !chunk[region_index..]
            .iter()
            .all(|b: &u8| b.is_ascii_alphanumeric())
    {
        Err(ParserError::InvalidSubdivision)
    } else {
        let suffix = match str::from_utf8(&chunk[region_index..]) {
            Ok(s) => s,
            Err(_) => return Err(ParserError::Unexpected),
        };
        Ok(UnicodeSubdivisionIdentifier {
            region: String::from(region),
            suffix: String::from(suffix),
        })
    }
}

fn region_index(chunk: &[u8]) -> Result<usize, ParserError> {
    if chunk[0..2].iter().all(|b| b.is_ascii_alphabetic()) {
        Ok(2)
    } else if chunk[0..3].iter().all(|b| b.is_ascii_digit()) {
        Ok(3)
    } else {
        Err(ParserError::InvalidSubdivision)
    }
}

#[test]
fn success_parse_unicode_subdivision_id() {
    // alpha region + suffix
    let subdivision = parse_unicode_subdivision_id("ussct").unwrap();
    assert_eq!("us", subdivision.region);
    assert_eq!("sct", subdivision.suffix);

    // digit region + suffix
    let subdivision = parse_unicode_subdivision_id("123abcd").unwrap();
    assert_eq!("123", subdivision.region);
    assert_eq!("abcd", subdivision.suffix);

    // Display trait implementation
    assert_eq!(
        "123abcd",
        format!("{}", parse_unicode_subdivision_id("123abcd").unwrap())
    );

    // PartialEq trait implementation
    assert_eq!(
        parse_unicode_subdivision_id("123abcd").unwrap(),
        parse_unicode_subdivision_id("123abcd").unwrap()
    );

    // FromStr trait implementation
    let subdivision: UnicodeSubdivisionIdentifier = "ussct".parse().unwrap();
    assert_eq!("us", subdivision.region);
    assert_eq!("sct", subdivision.suffix);
}

#[test]
fn fail_parse_unicode_subdivision_id() {
    // 2 characters
    assert_eq!(
        ParserError::InvalidSubdivision,
        parse_unicode_subdivision_id("ab").unwrap_err()
    );

    // 8 characters
    assert_eq!(
        ParserError::InvalidSubdivision,
        parse_unicode_subdivision_id("12312345").unwrap_err()
    );

    // invalid region
    assert_eq!(
        ParserError::InvalidSubdivision,
        parse_unicode_subdivision_id("1b123").unwrap_err()
    );

    // invalid suffix
    assert_eq!(
        ParserError::InvalidSubdivision,
        parse_unicode_subdivision_id("ab{}").unwrap_err()
    );
}
