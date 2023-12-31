pub use self::errors::ParserError;

pub mod errors;

#[derive(Debug)]
pub struct UnicodeLanguageId {
}

#[derive(Debug)]
pub enum ExtensionType {
    Unicode,
    Transformed,
    Pu,
    Other
}

#[derive(Debug)]
pub struct KeyPair(String, String);


#[derive(Debug)]
pub struct UnicodeExtension {
    pub keywords: KeyPair,
    pub attributes: Vec<String>
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
}

pub fn parse(locale: String) -> Result<UnicdeLocaleId, ParserError>{
    Ok(UnicdeLocaleId {
        lang: UnicodeLanguageId {},
    })
}
