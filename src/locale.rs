use crate::constants::SEP;
use crate::errors::ParserError;
use crate::extensions::{parse_extensions_from_iter, Extensions};
use crate::lang::{parse_unicode_language_id_from_iter, UnicodeLanguageIdentifier};
use crate::utils::split_str;

use std::fmt::{self};

#[derive(Debug)]
pub struct UnicdeLocaleIdentifier {
    pub language: UnicodeLanguageIdentifier,
    pub extensions: Extensions,
}

impl fmt::Display for UnicdeLocaleIdentifier {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut messages = vec![];
        messages.push(format!("{}", self.language));
        messages.push(format!("{}", self.extensions));
        f.write_str(&messages.join(&SEP.to_string()))?;
        Ok(())
    }
}

pub fn parse_unicode_locale_id(locale: &str) -> Result<UnicdeLocaleIdentifier, ParserError> {
    // check empty
    if locale.is_empty() {
        return Err(ParserError::Missing);
    }

    let mut iter = split_str(locale).peekable();
    let language = parse_unicode_language_id_from_iter(&mut iter, true)?;
    let extensions = parse_extensions_from_iter(&mut iter)?;

    Ok(UnicdeLocaleIdentifier {
        language,
        extensions,
    })
}

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
}

#[test]
fn fail_parse_unicode_locale_id() {
    // missing locale
    assert_eq!(
        ParserError::Missing,
        parse_unicode_locale_id("").unwrap_err()
    );
}
