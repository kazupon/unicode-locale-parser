use crate::constants::SEP;
use crate::errors::ParserError;
use crate::extensions::{parse_extensions_from_iter, Extensions};
use crate::lang::{parse_unicode_language_id_from_iter, UnicodeLanguageIdentifier};
use crate::shared::split_str;

use std::fmt::{self};
use std::str;
use std::str::FromStr;

#[derive(Debug)]
pub struct UnicodeLocaleIdentifier {
    pub language: UnicodeLanguageIdentifier,
    pub extensions: Extensions,
}

impl fmt::Display for UnicodeLocaleIdentifier {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut msg = vec![];
        msg.push(format!("{}", self.language));
        let extensions_msg = format!("{}", self.extensions);
        if !extensions_msg.is_empty() {
            msg.push(extensions_msg);
        }
        f.write_str(&msg.join(&SEP.to_string()))?;
        Ok(())
    }
}

impl FromStr for UnicodeLocaleIdentifier {
    type Err = ParserError;

    fn from_str(source: &str) -> Result<Self, Self::Err> {
        parse_unicode_locale_id(source)
    }
}

pub fn parse_unicode_locale_id(locale: &str) -> Result<UnicodeLocaleIdentifier, ParserError> {
    // check empty
    if locale.is_empty() {
        return Err(ParserError::Missing);
    }

    let mut iter = split_str(locale).peekable();
    let language = parse_unicode_language_id_from_iter(&mut iter)?;
    let extensions = parse_extensions_from_iter(&mut iter)?;

    Ok(UnicodeLocaleIdentifier {
        language,
        extensions,
    })
}

/*
 * Unit tests
 */

#[test]
fn success_parse_unicode_locale_id() {
    // full case
    let locale = parse_unicode_locale_id("ja-Latn-JP-macos-U-attr1-kz-value2-t-en-Latn-US-linux-t1-value1-value2-a-vue-rust-x-foo-123")
            .unwrap();
    assert_eq!("ja-Latn-JP-macos", format!("{}", locale.language));
    assert_eq!(
        "u-attr1-kz-value2-t-en-Latn-US-linux-t1-value1-value2-a-vue-rust-x-foo-123",
        format!("{}", locale.extensions)
    );

    // Display trait implementation
    assert_eq!(
        "ja-Latn-JP-macos-u-attr1-kz-value2-t-en-Latn-US-linux-t1-value1-value2-a-vue-rust-x-foo-123",
        format!("{}", parse_unicode_locale_id("ja-Latn-JP-macos-U-attr1-kz-value2-t-en-Latn-US-linux-t1-value1-value2-a-vue-rust-x-foo-123")
            .unwrap())
    );

    // FromStr trait implementation
    let result: UnicodeLocaleIdentifier = "ja-Latn-JP".parse().unwrap();
    assert_eq!("ja-Latn-JP", format!("{}", result));
}

#[test]
fn fail_parse_unicode_locale_id() {
    // missing locale
    assert_eq!(
        ParserError::Missing,
        parse_unicode_locale_id("").unwrap_err()
    );
}
