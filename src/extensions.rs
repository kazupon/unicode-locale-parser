use crate::errors::ParserError;
use crate::lang::UnicodeLanguageIdentifier;

#[derive(Debug)]
pub enum ExtensionType {
    UnicodeLocale,
    Transformed,
    Pu,
    Other,
}

#[derive(Debug)]
pub struct Extensions {
    pub values: Option<Vec<String>>,
}

#[derive(Debug)]
pub struct KeyPair(String, String);

#[derive(Debug)]
pub struct UnicodeLocaleExtensions {
    pub keyword: Vec<String>,
    pub attribute: Vec<String>,
}

#[derive(Debug)]
pub struct TransformedExtensions {
    pub tlang: Option<UnicodeLanguageIdentifier>,
    pub tfield: Vec<String>,
}

#[derive(Debug)]
pub struct PuExtensions {
    pub value: String,
}

#[derive(Debug)]
pub struct OtherExtensions {
    pub value: String,
}

pub fn parse_unicode_extensions(chunk: &str) -> Result<Extensions, ParserError> {
    Ok(Extensions {
        values: Some(vec![]),
    })
}
