use crate::lang::UnicodeLanguageIdentifier;

#[derive(Debug)]
pub struct TransformedExtensions {
    pub tlang: Option<UnicodeLanguageIdentifier>,
    pub tfield: Vec<String>,
}
