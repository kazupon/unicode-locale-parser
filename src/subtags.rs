use crate::errors::ParserError;

static LANG_ROOT: &str = "root";

pub fn get_language_subtag(subtag: &str) -> Result<&str, ParserError> {
    if LANG_ROOT.eq(subtag) {
        // 'root' is a special case
        Ok(LANG_ROOT)
    } else {
        // unicode_language_subtag
        // https://unicode.org/reports/tr35/#unicode_language_subtag
        let len = subtag.len();
        if !(2..=8).contains(&len) || len == 4 || !subtag.chars().all(|c| c.is_ascii_alphabetic()) {
            Err(ParserError::InvalidLanguage)
        } else {
            Ok(subtag)
        }
    }
}

pub fn get_script_subtag(subtag: &str) -> Result<&str, ParserError> {
    let len = subtag.len();
    // unicode_script_subtag
    // https://unicode.org/reports/tr35/#unicode_script_subtag
    if len != 4 || !subtag.chars().all(|c| c.is_ascii_alphabetic()) {
        Err(ParserError::InvalidSubtag)
    } else {
        Ok(subtag)
    }
}

pub fn get_region_subtag(subtag: &str) -> Result<&str, ParserError> {
    let len = subtag.len();
    // unicode_region_subtag
    // https://unicode.org/reports/tr35/#unicode_region_subtag
    if (len == 2 && subtag.chars().all(|c| c.is_ascii_alphabetic()))
        || (len == 3 && subtag.chars().all(|c| c.is_ascii_digit()))
    {
        Ok(subtag)
    } else {
        Err(ParserError::InvalidSubtag)
    }
}

/**
 * Unit tests
 */

#[test]
fn test_get_language_subtag_success() {
    // 'root'
    let result = get_language_subtag("root").unwrap();
    assert_eq!(result, "root");

    // language subtag only
    let result = get_language_subtag("en").unwrap();
    assert_eq!(result, "en");
}

#[test]
fn test_get_language_subtag_fail() {
    // 1 character
    let result = get_language_subtag("i");
    assert_eq!(result.err(), Some(ParserError::InvalidLanguage));

    // 4 characters
    let result = get_language_subtag("food");
    assert_eq!(result.err(), Some(ParserError::InvalidLanguage));

    // 9 characters
    let result = get_language_subtag("unicodela");
    assert_eq!(result.err(), Some(ParserError::InvalidLanguage));

    // not alphabet
    let result = get_language_subtag("12");
    assert_eq!(result.err(), Some(ParserError::InvalidLanguage));
}

#[test]
fn test_get_script_subtag_success() {
    let result = get_script_subtag("Latn").unwrap();
    assert_eq!(result, "Latn");
}

#[test]
fn test_get_script_subtag_fail() {
    // 3 character
    let result = get_script_subtag("foo");
    assert_eq!(result.err(), Some(ParserError::InvalidSubtag));

    // 5 characters
    let result = get_script_subtag("Japan");
    assert_eq!(result.err(), Some(ParserError::InvalidSubtag));

    // not alphabet
    let result = get_script_subtag("123");
    assert_eq!(result.err(), Some(ParserError::InvalidSubtag));
}

#[test]
fn test_get_region_subtag_success() {
    // ascii alphabet
    let result = get_region_subtag("JP").unwrap();
    assert_eq!(result, "JP");

    // 3 digit number
    let result = get_region_subtag("001").unwrap();
    assert_eq!(result, "001");
}

#[test]
fn test_get_region_subtag_fail() {
    // 1 character
    let result = get_region_subtag("J");
    assert_eq!(result.err(), Some(ParserError::InvalidSubtag));

    // 3 ascii characters
    let result = get_region_subtag("JPN");
    assert_eq!(result.err(), Some(ParserError::InvalidSubtag));

    // 4 digit characters
    let result = get_region_subtag("1234");
    assert_eq!(result.err(), Some(ParserError::InvalidSubtag));
}
