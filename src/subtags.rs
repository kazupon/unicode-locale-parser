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
    // unicode_script_subtag
    // https://unicode.org/reports/tr35/#unicode_script_subtag
    let len = subtag.len();
    if len != 4 || !subtag.chars().all(|c| c.is_ascii_alphabetic()) {
        Err(ParserError::InvalidSubtag)
    } else {
        Ok(subtag)
    }
}

pub fn get_region_subtag(subtag: &str) -> Result<&str, ParserError> {
    // unicode_region_subtag
    // https://unicode.org/reports/tr35/#unicode_region_subtag
    let len = subtag.len();
    if (len == 2 && subtag.chars().all(|c| c.is_ascii_alphabetic()))
        || (len == 3 && subtag.chars().all(|c| c.is_ascii_digit()))
    {
        Ok(subtag)
    } else {
        Err(ParserError::InvalidSubtag)
    }
}

pub fn get_variant_subtag(subtag: &str) -> Result<&str, ParserError> {
    // unicode_variant_subtag
    // https://unicode.org/reports/tr35/#unicode_variant_subtag
    let len = subtag.len();
    if !(4..=8).contains(&len) {
        return Err(ParserError::InvalidSubtag);
    }

    if len >= 5 && !subtag.chars().all(|c| c.is_ascii_alphanumeric()) {
        return Err(ParserError::InvalidSubtag);
    } else if len == 4 {
        let subtag_bytes = subtag.as_bytes();
        if !subtag_bytes[0].is_ascii_digit()
            || !subtag_bytes[1..]
                .iter()
                .all(|c: &u8| c.is_ascii_alphanumeric())
        {
            return Err(ParserError::InvalidSubtag);
        }
    }

    Ok(subtag)
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

#[test]
fn test_get_variant_subtag_success() {
    // 4 characters with digit
    let result = get_variant_subtag("1996").unwrap();
    assert_eq!(result, "1996");

    // 4 characters with digit & alphabet
    let result = get_variant_subtag("1ABC").unwrap();
    assert_eq!(result, "1ABC");

    // 5 characters with alphabet and digit
    let result = get_variant_subtag("abcd1").unwrap();
    assert_eq!(result, "abcd1");

    // 8 characters with alphabet and digit
    let result = get_variant_subtag("abcdefgh").unwrap();
    assert_eq!(result, "abcdefgh");
}

#[test]
fn test_get_variant_subtag_fail() {
    // 3 characters
    let result = get_variant_subtag("abc");
    assert_eq!(result.err(), Some(ParserError::InvalidSubtag));

    // 9 characters
    let result = get_variant_subtag("abcdefghi");
    assert_eq!(result.err(), Some(ParserError::InvalidSubtag));

    // 4 characters with alphabet
    let result = get_variant_subtag("aBCD");
    assert_eq!(result.err(), Some(ParserError::InvalidSubtag));
}
