use crate::errors::ParserError;
use crate::subtags::{get_language_subtag, get_script_subtag};

#[derive(Debug)]
pub struct UnicodeLanguageId {
    pub language: String,
    pub script: Option<String>,
    pub region: Option<String>,
    pub variants: Vec<String>, // TODO: should re-design
}

pub fn parse_unicode_language_id(chunk: &str) -> Result<UnicodeLanguageId, ParserError> {
    // check empty
    if chunk.is_empty() {
        return Err(ParserError::MissingLanguage);
    }

    let mut iter = chunk.split(|c| c == '-' || c == '_').peekable();

    // language subtag
    let language = if let Some(lang) = iter.next() {
        get_language_subtag(lang)?
    } else {
        return Err(ParserError::Unexpected);
    };

    let mut script = None;
    let mut region = None;
    let mut variants = vec![];

    Ok(UnicodeLanguageId {
        language: String::from(language),
        script,
        region,
        variants,
    })
}

/**
 * Unit tests
 */

#[test]
fn test_language_id() {
    // 'root'
    let result = parse_unicode_language_id("root").unwrap();
    assert_eq!(result.language, "root");
    assert_eq!(result.script, None);
    assert_eq!(result.region, None);
    assert_eq!(result.variants.is_empty(), true);

    // language subtag only
    let result = parse_unicode_language_id("en").unwrap();
    assert_eq!(result.language, "en");
}

#[test]
fn test_language_id_missing() {
    let result = parse_unicode_language_id("");
    assert_eq!(result.err(), Some(ParserError::MissingLanguage));
}
