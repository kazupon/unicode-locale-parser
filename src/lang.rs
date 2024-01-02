use crate::errors::ParserError;

#[derive(Debug)]
pub struct UnicodeLanguageId {
    pub language: String,
    pub script: Option<String>,
    pub region: Option<String>,
    pub variants: Vec<String>, // TODO: should re-design
}

pub fn parse_unicode_language_id(chunk: &str) -> Result<UnicodeLanguageId, ParserError> {
    if chunk.is_empty() {
        return Err(ParserError::MissingLangugage);
    }
    // let mut iter = chunk.split(|c| c == '-' || c == '_').peekable();
    Ok(UnicodeLanguageId {
        language: String::from("en"),
        script: None,
        region: None,
        variants: vec![],
    })
}

#[test]
fn test_language_id_missing() {
    let result = parse_unicode_language_id("");
    assert_eq!(result.err(), Some(ParserError::MissingLangugage));
}
