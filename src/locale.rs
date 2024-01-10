use crate::constants::SEP;
use crate::errors::ParserError;
use crate::extensions::{parse_extensions_from_iter, Extensions};
use crate::lang::{parse_unicode_language_id_from_iter, UnicodeLanguageIdentifier};
use crate::shared::split_str;

use std::fmt::{self};
use std::str;
use std::str::FromStr;

#[derive(Debug)]
pub struct UnicodeLocaleIdentifier {
  pub language: UnicodeLanguageIdentifier,
  pub extensions: Extensions,
}

impl fmt::Display for UnicodeLocaleIdentifier {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    let mut msg = vec![];
    msg.push(format!("{}", self.language));
    let extensions_msg = format!("{}", self.extensions);
    if !extensions_msg.is_empty() {
      msg.push(extensions_msg);
    }
    f.write_str(&msg.join(&SEP.to_string()))?;
    Ok(())
  }
}

impl FromStr for UnicodeLocaleIdentifier {
  type Err = ParserError;

  fn from_str(source: &str) -> Result<Self, Self::Err> {
    parse_unicode_locale_id(source)
  }
}

/// Parse the given string as an Unicode Locale Identifier.
///
/// This function parses according to [`unicode_locale_id` EBNF defined in UTS #35](https://unicode.org/reports/tr35/#unicode_locale_id)
///
/// # Examples
///
/// ```
/// use unicode_locale_parser::parse_locale_id;
///
/// let locale = parse_locale_id("en-US-u-hc-h12").unwrap();
/// assert_eq!("en", locale.language.language);
/// assert_eq!(None, locale.language.script);
/// assert_eq!(Some("US".to_string()), locale.language.region);
/// assert_eq!(None, locale.language.variants);
/// let u = locale.extensions.unicode_locale.unwrap();
/// assert_eq!(
///     &vec!["h12".to_string()],
///     u.get(0).unwrap().ufield.get("hc").unwrap()
/// );
/// ```
///
/// # Errors
///
/// This function returns an error in the following cases:
///
/// - [`ParserError::Missing`] if the given locale id is empty.
/// - [`ParserError::InvalidLanguage`] if the given locale id is not a valid language identifier.
/// - [`ParserError::InvalidSubtag`] if the given locale id is not a valid subtag.
/// - [`ParserError::InvalidExtension`] if the given locale id is not a valid unicode extensions
pub fn parse_unicode_locale_id(locale_id: &str) -> Result<UnicodeLocaleIdentifier, ParserError> {
  // check empty
  if locale_id.is_empty() {
    return Err(ParserError::Missing);
  }

  let mut iter = split_str(locale_id).peekable();
  let language = parse_unicode_language_id_from_iter(&mut iter)?;
  let extensions = parse_extensions_from_iter(&mut iter)?;

  Ok(UnicodeLocaleIdentifier {
    language,
    extensions,
  })
}

/*
 * Unit tests
 */

#[test]
fn success_parse_unicode_locale_id() {
  // basic
  let locale = parse_unicode_locale_id("en-US-u-hc-h12").unwrap();
  assert_eq!("en", locale.language.language);
  assert_eq!(None, locale.language.script);
  assert_eq!(Some("US".to_string()), locale.language.region);
  assert_eq!(None, locale.language.variants);
  let u = locale.extensions.unicode_locale.unwrap();
  assert_eq!(
    &vec!["h12".to_string()],
    u.get(0).unwrap().ufield.get("hc").unwrap()
  );

  // full case
  let locale = parse_unicode_locale_id(
    "ja-Latn-JP-macos-U-attr1-kz-value2-t-en-Latn-US-linux-t1-value1-value2-a-vue-rust-x-foo-123",
  )
  .unwrap();
  assert_eq!("ja-Latn-JP-macos", format!("{}", locale.language));
  assert_eq!(
    "u-attr1-kz-value2-t-en-Latn-US-linux-t1-value1-value2-a-vue-rust-x-foo-123",
    format!("{}", locale.extensions)
  );

  // Display trait implementation
  assert_eq!(
        "ja-Latn-JP-macos-u-attr1-kz-value2-t-en-Latn-US-linux-t1-value1-value2-a-vue-rust-x-foo-123",
        format!("{}", parse_unicode_locale_id("ja-Latn-JP-macos-U-attr1-kz-value2-t-en-Latn-US-linux-t1-value1-value2-a-vue-rust-x-foo-123")
            .unwrap())
    );

  // FromStr trait implementation
  let result: UnicodeLocaleIdentifier = "ja-Latn-JP".parse().unwrap();
  assert_eq!("ja-Latn-JP", format!("{}", result));
}

#[test]
fn fail_parse_unicode_locale_id() {
  // missing locale
  assert_eq!(
    ParserError::Missing,
    parse_unicode_locale_id("").unwrap_err()
  );
}
