use crate::constants::{LANG_EMPTY, LANG_ROOT, LANG_UND};
use crate::errors::ParserError;

pub fn is_language_subtag(subtag: &[u8]) -> bool {
  let len = subtag.len();
  (2..=8).contains(&len) && len != 4 && subtag.iter().all(|b| b.is_ascii_alphabetic())
}

pub fn language_subtag(subtag: &str) -> Result<&str, ParserError> {
  // unicode_language_subtag
  // https://unicode.org/reports/tr35/#unicode_language_subtag

  // 'root' is a special case
  if matches!(subtag, LANG_ROOT) {
    return Ok(LANG_EMPTY);
  }

  if !is_language_subtag(subtag.as_bytes()) {
    return Err(ParserError::InvalidLanguage);
  }

  if matches!(subtag, LANG_UND) {
    Ok(LANG_EMPTY)
  } else {
    Ok(subtag)
  }
}

fn is_scritp_subtag(subtag: &[u8]) -> bool {
  subtag.len() == 4 && subtag.iter().all(|b| b.is_ascii_alphabetic())
}

pub fn script_subtag(subtag: &str) -> Result<&str, ParserError> {
  // unicode_script_subtag
  // https://unicode.org/reports/tr35/#unicode_script_subtag

  if is_scritp_subtag(subtag.as_bytes()) {
    Ok(subtag)
  } else {
    Err(ParserError::InvalidSubtag)
  }
}

fn is_region_subtag(subtag: &[u8]) -> bool {
  let len = subtag.len();
  len == 2 && subtag.iter().all(|b| b.is_ascii_alphabetic())
    || len == 3 && subtag.iter().all(|b| b.is_ascii_digit())
}

pub fn region_subtag(subtag: &str) -> Result<&str, ParserError> {
  // unicode_region_subtag
  // https://unicode.org/reports/tr35/#unicode_region_subtag

  if is_region_subtag(subtag.as_bytes()) {
    Ok(subtag)
  } else {
    Err(ParserError::InvalidSubtag)
  }
}

fn is_variant_subtag(subtag: &[u8]) -> bool {
  let len = subtag.len();
  (4..=8).contains(&len)
    && ((len >= 5 && subtag.iter().all(|b| b.is_ascii_alphanumeric()))
      || (len == 4 && subtag[0].is_ascii_digit())
        && subtag[1..].iter().all(|b: &u8| b.is_ascii_alphanumeric()))
}

pub fn variant_subtag(subtag: &str) -> Result<&str, ParserError> {
  // unicode_variant_subtag
  // https://unicode.org/reports/tr35/#unicode_variant_subtag

  if is_variant_subtag(subtag.as_bytes()) {
    Ok(subtag)
  } else {
    Err(ParserError::InvalidSubtag)
  }
}

/**
 * Unit tests
 */

#[test]
fn success_language_subtag() {
  // 'root'
  assert_eq!(LANG_EMPTY, language_subtag("root").unwrap());

  // language subtag only
  assert_eq!("en", language_subtag("en").unwrap());

  // 3 characters
  assert_eq!("jpn", language_subtag("jpn").unwrap());

  // 'und'
  assert_eq!(LANG_EMPTY, language_subtag("und").unwrap());
}

#[test]
fn fail_get_language_subtag() {
  // 1 character
  assert_eq!(
    ParserError::InvalidLanguage,
    language_subtag("i").unwrap_err()
  );

  // 4 characters
  assert_eq!(
    ParserError::InvalidLanguage,
    language_subtag("food").unwrap_err()
  );

  // 9 characters
  assert_eq!(
    ParserError::InvalidLanguage,
    language_subtag("unicodela").unwrap_err()
  );

  // not alphabet
  assert_eq!(
    ParserError::InvalidLanguage,
    language_subtag("12").unwrap_err()
  );
}

#[test]
fn success_script_subtag() {
  assert_eq!("Latn", script_subtag("Latn").unwrap());
}

#[test]
fn fail_script_subtag() {
  // 3 character
  assert_eq!(
    ParserError::InvalidSubtag,
    script_subtag("foo").unwrap_err()
  );

  // 5 characters
  assert_eq!(
    ParserError::InvalidSubtag,
    script_subtag("Japan").unwrap_err()
  );

  // not alphabet
  assert_eq!(
    ParserError::InvalidSubtag,
    script_subtag("123").unwrap_err()
  );
}

#[test]
fn success_region_subtag() {
  // ascii alphabet
  assert_eq!("JP", region_subtag("JP").unwrap());

  // 3 digit number
  assert_eq!("001", region_subtag("001").unwrap());
}

#[test]
fn fail_region_subtag() {
  // 1 character
  assert_eq!(ParserError::InvalidSubtag, region_subtag("J").unwrap_err());

  // 3 ascii characters
  assert_eq!(
    ParserError::InvalidSubtag,
    region_subtag("JPN").unwrap_err()
  );

  // 4 digit characters
  assert_eq!(
    ParserError::InvalidSubtag,
    region_subtag("1234").unwrap_err()
  );
}

#[test]
fn success_variant_subtag() {
  // 4 characters with digit
  assert_eq!("1996", variant_subtag("1996").unwrap());

  // 4 characters with digit & alphabet
  assert_eq!("1ABC", variant_subtag("1ABC").unwrap());

  // 5 characters with alphabet and digit
  assert_eq!("abcd1", variant_subtag("abcd1").unwrap());

  // 8 characters with alphabet and digit
  assert_eq!("abcdefgh", variant_subtag("abcdefgh").unwrap());
}

#[test]
fn fail_variant_subtag() {
  // 3 characters
  assert_eq!(
    ParserError::InvalidSubtag,
    variant_subtag("abc").unwrap_err()
  );

  // 9 characters
  assert_eq!(
    ParserError::InvalidSubtag,
    variant_subtag("abcdefghi").unwrap_err()
  );

  // 4 characters with alphabet
  assert_eq!(
    ParserError::InvalidSubtag,
    variant_subtag("aBCD").unwrap_err()
  );
}
