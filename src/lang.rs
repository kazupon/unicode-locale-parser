use crate::errors::ParserError;

#[derive(Debug)]
pub struct UnicodeLanguageId {
    pub language: String,
    pub script: Option<String>,
    pub region: Option<String>,
    pub variants: Vec<String>, // TODO: should re-design
}

static LANG_ROOT: &str = "root";

fn get_language(lang: &str) -> Result<&str, ParserError> {
    if LANG_ROOT.eq(lang) {
        // 'root' is a special case
        Ok(LANG_ROOT)
    } else {
        // unicode_language_subtag
        // https://unicode.org/reports/tr35/#unicode_language_subtag
        let len = lang.len();
        if !(2..=8).contains(&len) || len == 4 || !lang.chars().all(|c| c.is_ascii_alphabetic()) {
            Err(ParserError::InvalidLanguage)
        } else {
            Ok(lang)
        }
    }
}

pub fn parse_unicode_language_id(chunk: &str) -> Result<UnicodeLanguageId, ParserError> {
    if chunk.is_empty() {
        return Err(ParserError::MissingLanguage);
    }
    let mut iter = chunk.split(|c| c == '-' || c == '_').peekable();

    // language subtag
    let language = if let Some(lang) = iter.next() {
        get_language(lang)?
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

#[test]
fn test_invalid_language_id() {
    // one character
    let result = parse_unicode_language_id("i");
    assert_eq!(result.err(), Some(ParserError::InvalidLanguage));

    // 4 characters
    let result = parse_unicode_language_id("food");
    assert_eq!(result.err(), Some(ParserError::InvalidLanguage));

    // 9 characters
    let result = parse_unicode_language_id("unicodela");
    assert_eq!(result.err(), Some(ParserError::InvalidLanguage));

    // not alphabet
    let result = parse_unicode_language_id("12");
    assert_eq!(result.err(), Some(ParserError::InvalidLanguage));
}
