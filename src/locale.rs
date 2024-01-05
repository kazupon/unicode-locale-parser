use crate::errors::ParserError;
use crate::lang::{parse_unicode_language_id_from_iter, UnicodeLanguageIdentifier};

#[derive(Debug)]
pub enum ExtensionType {
    Unicode,
    Transformed,
    Pu,
    Other,
}

#[derive(Debug)]
pub struct KeyPair(String, String);

#[derive(Debug)]
pub struct UnicodeExtension {
    pub keywords: KeyPair,
    pub attributes: Vec<String>,
}

#[derive(Debug)]
pub struct TransformedExtension {
    pub fields: KeyPair,
    pub lang: Option<UnicodeLanguageIdentifier>,
}

#[derive(Debug)]
pub struct PuExtension {
    pub value: String,
}

#[derive(Debug)]
pub struct OtherExtension {
    pub value: String,
}

#[derive(Debug)]
pub struct UnicdeLocaleIdentifier {
    pub language: UnicodeLanguageIdentifier,
    // pub extensions: Option(Some<Vec<ExtensionType>),
}

pub fn parse_unicode_locale_id(locale: &str) -> Result<UnicdeLocaleIdentifier, ParserError> {
    // check empty
    if locale.is_empty() {
        return Err(ParserError::MissingLocale);
    }

    let mut iter = locale.split(|c| c == '-' || c == '_').peekable();
    let language = parse_unicode_language_id_from_iter(&mut iter)?;

    Ok(UnicdeLocaleIdentifier { language })
}

#[test]
fn success_parse_unicode_locale_id() {
    // full case
    let result = parse_unicode_locale_id("en-Latn-US-macos").unwrap();
    assert_eq!(result.language.to_string(), "en-Latn-US-macos");
}

#[test]
fn fail_parse_unicode_locale_id() {
    // missing locale
    let result = parse_unicode_locale_id("");
    assert_eq!(result.err(), Some(ParserError::MissingLocale));
}
