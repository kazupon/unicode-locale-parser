use crate::constants::SEP;
use crate::errors::ParserError;
use crate::shared::split_str;

use std::fmt::{self};
use std::iter::Peekable;
use std::str::FromStr;

#[derive(Debug, PartialEq)]
pub struct UnicodeMeasureUnit {
    pub values: Vec<String>,
}

impl fmt::Display for UnicodeMeasureUnit {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut messages = vec![];
        for value in &self.values {
            messages.push(format!("{}", value));
        }
        write!(f, "{}", messages.join(&SEP.to_string()))?;
        Ok(())
    }
}

impl FromStr for UnicodeMeasureUnit {
    type Err = ParserError;

    fn from_str(source: &str) -> Result<Self, Self::Err> {
        parse_unicode_measure_unit(source)
    }
}

pub fn parse_unicode_measure_unit(chunk: &str) -> Result<UnicodeMeasureUnit, ParserError> {
    if chunk.is_empty() {
        return Err(ParserError::Missing);
    }

    parse_unicode_measure_unit_from_iter(&mut split_str(chunk).peekable())
}

fn parse_unicode_measure_unit_from_iter<'a>(
    iter: &mut Peekable<impl Iterator<Item = &'a str>>,
) -> Result<UnicodeMeasureUnit, ParserError> {
    // unicode_measure_unit
    // https://unicode.org/reports/tr35/#unicode_measure_unit
    let mut values = vec![];

    while let Some(subtag) = iter.peek() {
        let subtag_bytes = subtag.as_bytes();

        if !(3..=8).contains(&subtag_bytes.len())
            || !subtag_bytes.iter().all(|b: &u8| b.is_ascii_alphanumeric())
        {
            return Err(ParserError::InvalidSubtag);
        }

        values.push(subtag.to_string());
        iter.next();
    }

    let values = if values.is_empty() {
        return Err(ParserError::Missing);
    } else {
        values
    };

    Ok(UnicodeMeasureUnit { values })
}

/*
 * Unit tests
 */

#[test]
fn success_parse_unicode_measure_unit() {
    // basic
    let measure = parse_unicode_measure_unit("area-hectare").unwrap();
    assert_eq!(vec!["area", "hectare"], measure.values);

    // Display trait implementation
    assert_eq!(
        "area-hectare",
        format!("{}", parse_unicode_measure_unit("area-hectare").unwrap())
    );

    // PartialEq trait implementation
    assert_eq!(
        parse_unicode_measure_unit("area-hectare").unwrap(),
        parse_unicode_measure_unit("area-hectare").unwrap()
    );

    // FromStr trait implementation
    let measure: UnicodeMeasureUnit = "area-hectare".parse().unwrap();
    assert_eq!(vec!["area", "hectare"], measure.values);
}

#[test]
fn fail_parse_unicode_measure_unit() {
    // missing
    assert_eq!(
        ParserError::Missing,
        parse_unicode_measure_unit("").unwrap_err()
    );

    // invalid subtag
    assert_eq!(
        ParserError::InvalidSubtag,
        parse_unicode_measure_unit("acceleration-g-force").unwrap_err()
    );
}
