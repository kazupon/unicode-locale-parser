use crate::errors::ParserError;
use crate::lang::UnicodeLanguageId;

pub mod errors;
mod lang;

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
    pub lang: Option<UnicodeLanguageId>,
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
pub struct UnicdeLocaleId {
    pub lang: UnicodeLanguageId,
    // pub extensions: Option(Some<Vec<ExtensionType>),
}

pub fn parse(locale: &str) -> Result<UnicdeLocaleId, ParserError> {
    Ok(UnicdeLocaleId {
        lang: UnicodeLanguageId {
            language: String::from("en"),
            script: None,
            region: None,
            variants: vec![],
        },
    })
}
