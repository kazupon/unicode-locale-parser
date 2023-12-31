use crate::errors::ParserError;

#[derive(Debug)]
pub struct UnicodeLanguageId {
    pub language: String,
    pub script: Option<String>,
    pub region: Option<String>,
    pub variants: Vec<String>,
}

pub fn parse_unicode_language_id(chunk: &str) -> Result<UnicodeLanguageId, ParserError> {
    Ok(UnicodeLanguageId { language: String::from("en"), script: None, region: None, variants: vec![] })
}