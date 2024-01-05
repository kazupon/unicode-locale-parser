use crate::errors::ParserError;
use crate::lang::{parse_unicode_language_id_from_iter, UnicodeLanguageIdentifier};

#[derive(Debug)]
pub struct UnicdeLocaleIdentifier {
    pub language: UnicodeLanguageIdentifier,
    // pub extensions: Option(Some<Vec<ExtensionType>),
}

pub fn parse_unicode_locale_id(locale: &str) -> Result<UnicdeLocaleIdentifier, ParserError> {
    // check empty
    if locale.is_empty() {
        return Err(ParserError::Missing);
    }

    let mut iter = locale.split(|c| c == '-' || c == '_').peekable();
    let language = parse_unicode_language_id_from_iter(&mut iter, true)?;

    Ok(UnicdeLocaleIdentifier { language })
}

#[test]
fn success_parse_unicode_locale_id() {
    // full case
    assert_eq!(
        "en-Latn-US-macos",
        parse_unicode_locale_id("en-Latn-US-macos")
            .unwrap()
            .language
            .to_string()
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
