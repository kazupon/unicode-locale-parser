use crate::constants::SEP;
use crate::errors::ParserError;
use crate::extensions::ExtensionKind;

use std::collections::BTreeMap;
use std::fmt::{self, Debug, Write};
use std::iter::Peekable;

#[derive(Debug)]
pub struct UnicodeLocaleExtensions {
    pub attribute: Vec<String>,
    pub ufield: BTreeMap<String, Vec<String>>,
}

impl fmt::Display for UnicodeLocaleExtensions {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", ExtensionKind::UnicodeLocale)?;
        for attribute in &self.attribute {
            f.write_char(SEP)?;
            f.write_str(attribute)?;
        }
        for (key, values) in &self.ufield {
            f.write_char(SEP)?;
            f.write_str(key)?;
            for value in values {
                f.write_char(SEP)?;
                f.write_str(value)?;
            }
        }
        Ok(())
    }
}

pub fn parse_unicode_locale_extensions<'a>(
    mut iter: &mut Peekable<impl Iterator<Item = &'a str>>,
) -> Result<UnicodeLocaleExtensions, ParserError> {
    // unicode_locale_extensions
    // https://unicode.org/reports/tr35/#unicode_locale_extensions

    let mut attribute = vec![];
    let mut ufield = BTreeMap::new();
    let mut ukey: Option<String> = None;
    let mut uvalue: Vec<String> = vec![];

    while let Some(subtag) = iter.peek() {
        let subtag_bytes = subtag.as_bytes();
        let len = subtag_bytes.len();
        if len == 1 {
            break;
        } else if len == 2
            && subtag_bytes[0].is_ascii_alphanumeric()
            && subtag_bytes[1].is_ascii_alphabetic()
        {
            // for ukey
            if let Some(ukey) = ukey {
                if !ufield.contains_key(&ukey) {
                    ufield.insert(ukey.clone(), vec![]);
                }
                let values = ufield.get_mut(&ukey).unwrap();
                for value in uvalue {
                    values.push(value);
                }
                uvalue = vec![];
            }
            ukey = Some(subtag.to_string());
            iter.next();
        } else if (3..=8).contains(&len) && subtag_bytes.iter().all(|c| c.is_ascii_alphanumeric()) {
            if ukey.is_some() {
                // for uvalue
                uvalue.push(subtag.to_string());
            } else {
                // for attribute
                uvalue.push(subtag.to_string());
            }
            iter.next();
        } else {
            return Err(ParserError::InvalidSubtag);
        }
    }

    if let Some(ukey) = ukey {
        if !ufield.contains_key(&ukey) {
            ufield.insert(ukey.clone(), vec![]);
        }
        let values = ufield.get_mut(&ukey).unwrap();
        for value in uvalue {
            values.push(value);
        }
    }

    Ok(UnicodeLocaleExtensions { attribute, ufield })
}

/*
 * Unit tests
 */

#[allow(unused_imports)] // for unit tests
use crate::utils::split_str;

#[test]
fn success_unicode_locale_extensions() {
    // basic case
    let mut iter = split_str("1k-value1-attr-ky-value2").peekable();
    assert_eq!(
        "u-1k-value1-attr-ky-value2",
        format!("{}", parse_unicode_locale_extensions(&mut iter).unwrap())
    );

    // no attribute
    let mut iter = split_str("1k-value1-ky-value2").peekable();
    assert_eq!(
        "u-1k-value1-ky-value2",
        format!("{}", parse_unicode_locale_extensions(&mut iter).unwrap())
    );

    // attribute multiple
    let mut iter = split_str("1k-value1-attr1-attr2-ky-value2").peekable();
    assert_eq!(
        "u-1k-value1-attr1-attr2-ky-value2",
        format!("{}", parse_unicode_locale_extensions(&mut iter).unwrap())
    );

    // uvalue multiple
    let mut iter = split_str("ky-value1-value2").peekable();
    assert_eq!(
        "u-ky-value1-value2",
        format!("{}", parse_unicode_locale_extensions(&mut iter).unwrap())
    );

    // no uvalue
    let mut iter = split_str("ky").peekable();
    assert_eq!(
        "u-ky",
        format!("{}", parse_unicode_locale_extensions(&mut iter).unwrap())
    );
}

#[test]
fn fail_unicode_locale_extensions() {
    // invalid ukey
    let mut iter = split_str("k1").peekable();
    assert_eq!(
        ParserError::InvalidSubtag,
        parse_unicode_locale_extensions(&mut iter).unwrap_err()
    );

    // invalid uvalue
    let mut iter = split_str("ky-{}").peekable();
    assert_eq!(
        ParserError::InvalidSubtag,
        parse_unicode_locale_extensions(&mut iter).unwrap_err()
    );

    // invalid attribute
    let mut iter = split_str("ky-value1-{?}").peekable();
    assert_eq!(
        ParserError::InvalidSubtag,
        parse_unicode_locale_extensions(&mut iter).unwrap_err()
    );
}
