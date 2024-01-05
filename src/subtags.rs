use crate::errors::ParserError;

const LANG_ROOT: &str = "root";
const LANG_EMPTY: &str = "";

pub fn language_subtag(subtag: &str) -> Result<&str, ParserError> {
    // unicode_language_subtag
    // https://unicode.org/reports/tr35/#unicode_language_subtag

    // 'root' is a special case
    if LANG_ROOT.eq(subtag) {
        return Ok(LANG_EMPTY);
    }

    let len = subtag.len();
    if !(2..=8).contains(&len)
        || len == 4
        || !subtag.as_bytes().iter().all(|b| b.is_ascii_alphabetic())
    {
        return Err(ParserError::InvalidLanguage);
    }

    if "und" == subtag {
        Ok(LANG_EMPTY)
    } else {
        Ok(subtag)
    }
}

pub fn script_subtag(subtag: &str) -> Result<&str, ParserError> {
    // unicode_script_subtag
    // https://unicode.org/reports/tr35/#unicode_script_subtag

    let len = subtag.len();
    if len != 4 || !subtag.as_bytes().iter().all(|b| b.is_ascii_alphabetic()) {
        Err(ParserError::InvalidSubtag)
    } else {
        Ok(subtag)
    }
}

pub fn region_subtag(subtag: &str) -> Result<&str, ParserError> {
    // unicode_region_subtag
    // https://unicode.org/reports/tr35/#unicode_region_subtag

    let len = subtag.len();
    if (len == 2 && subtag.as_bytes().iter().all(|b| b.is_ascii_alphabetic()))
        || (len == 3 && subtag.as_bytes().iter().all(|b| b.is_ascii_digit()))
    {
        Ok(subtag)
    } else {
        Err(ParserError::InvalidSubtag)
    }
}

pub fn variant_subtag(subtag: &str) -> Result<&str, ParserError> {
    // unicode_variant_subtag
    // https://unicode.org/reports/tr35/#unicode_variant_subtag

    let len = subtag.len();
    if !(4..=8).contains(&len) {
        return Err(ParserError::InvalidSubtag);
    }

    let subtag_bytes = subtag.as_bytes();
    if len >= 5 && !subtag_bytes.iter().all(|b| b.is_ascii_alphanumeric())
        || len == 4 && !subtag_bytes[0].is_ascii_digit()
        || !subtag_bytes[1..]
            .iter()
            .all(|b: &u8| b.is_ascii_alphanumeric())
    {
        return Err(ParserError::InvalidSubtag);
    }

    Ok(subtag)
}

/**
 * Unit tests
 */

#[test]
fn success_language_subtag() {
    // 'root'
    let result = language_subtag("root").unwrap();
    assert_eq!(result, LANG_EMPTY);

    // language subtag only
    let result = language_subtag("en").unwrap();
    assert_eq!(result, "en");

    // 3 characters
    let result = language_subtag("jpn").unwrap();
    assert_eq!(result, "jpn");

    // 'und'
    let result = language_subtag("und").unwrap();
    assert_eq!(result, LANG_EMPTY);
}

#[test]
fn fail_get_language_subtag() {
    // 1 character
    let result = language_subtag("i");
    assert_eq!(result.err(), Some(ParserError::InvalidLanguage));

    // 4 characters
    let result = language_subtag("food");
    assert_eq!(result.err(), Some(ParserError::InvalidLanguage));

    // 9 characters
    let result = language_subtag("unicodela");
    assert_eq!(result.err(), Some(ParserError::InvalidLanguage));

    // not alphabet
    let result = language_subtag("12");
    assert_eq!(result.err(), Some(ParserError::InvalidLanguage));
}

#[test]
fn success_script_subtag() {
    let result = script_subtag("Latn").unwrap();
    assert_eq!(result, "Latn");
}

#[test]
fn fail_script_subtag() {
    // 3 character
    let result = script_subtag("foo");
    assert_eq!(result.err(), Some(ParserError::InvalidSubtag));

    // 5 characters
    let result = script_subtag("Japan");
    assert_eq!(result.err(), Some(ParserError::InvalidSubtag));

    // not alphabet
    let result = script_subtag("123");
    assert_eq!(result.err(), Some(ParserError::InvalidSubtag));
}

#[test]
fn success_region_subtag() {
    // ascii alphabet
    let result = region_subtag("JP").unwrap();
    assert_eq!(result, "JP");

    // 3 digit number
    let result = region_subtag("001").unwrap();
    assert_eq!(result, "001");
}

#[test]
fn fail_region_subtag() {
    // 1 character
    let result = region_subtag("J");
    assert_eq!(result.err(), Some(ParserError::InvalidSubtag));

    // 3 ascii characters
    let result = region_subtag("JPN");
    assert_eq!(result.err(), Some(ParserError::InvalidSubtag));

    // 4 digit characters
    let result = region_subtag("1234");
    assert_eq!(result.err(), Some(ParserError::InvalidSubtag));
}

#[test]
fn success_variant_subtag() {
    // 4 characters with digit
    let result = variant_subtag("1996").unwrap();
    assert_eq!(result, "1996");

    // 4 characters with digit & alphabet
    let result = variant_subtag("1ABC").unwrap();
    assert_eq!(result, "1ABC");

    // 5 characters with alphabet and digit
    let result = variant_subtag("abcd1").unwrap();
    assert_eq!(result, "abcd1");

    // 8 characters with alphabet and digit
    let result = variant_subtag("abcdefgh").unwrap();
    assert_eq!(result, "abcdefgh");
}

#[test]
fn fail_variant_subtag() {
    // 3 characters
    let result = variant_subtag("abc");
    assert_eq!(result.err(), Some(ParserError::InvalidSubtag));

    // 9 characters
    let result = variant_subtag("abcdefghi");
    assert_eq!(result.err(), Some(ParserError::InvalidSubtag));

    // 4 characters with alphabet
    let result = variant_subtag("aBCD");
    assert_eq!(result.err(), Some(ParserError::InvalidSubtag));
}
