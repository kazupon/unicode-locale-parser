use crate::constants::SEP;
use crate::errors::ParserError;
use crate::extensions::ExtensionKind;
use crate::lang::{parse_unicode_language_id_from_iter, UnicodeLanguageIdentifier};
use crate::subtags::is_language_subtag;

use std::collections::BTreeMap;
use std::fmt::{self, Debug, Write};
use std::iter::Peekable;

#[derive(Debug)]
pub struct TransformedExtensions {
  pub tlang: Option<UnicodeLanguageIdentifier>,
  pub tfield: BTreeMap<String, Vec<String>>,
}

impl fmt::Display for TransformedExtensions {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "{}", ExtensionKind::Transformed)?;
    if let Some(tlang) = &self.tlang {
      f.write_char(SEP)?;
      write!(f, "{}", tlang)?;
    }
    for (key, values) in &self.tfield {
      f.write_char(SEP)?;
      f.write_str(key)?;
      for value in values {
        f.write_char(SEP)?;
        f.write_str(value)?;
      }
    }
    Ok(())
  }
}

pub fn parse_transformed_extensions<'a>(
  iter: &mut Peekable<impl Iterator<Item = &'a str>>,
) -> Result<TransformedExtensions, ParserError> {
  // transformed_extensions
  // https://unicode.org/reports/tr35/#transformed_extensions
  let mut tlang = None;
  let mut tfield = BTreeMap::new();
  let mut tkey: Option<String> = None;
  let mut tvalue: Vec<String> = vec![];

  while let Some(subtag) = iter.peek() {
    let subtag_bytes = subtag.as_bytes();
    let len = subtag_bytes.len();
    if len == 1 {
      break;
    } else if len == 2 && subtag_bytes[0].is_ascii_alphabetic() && subtag_bytes[1].is_ascii_digit()
    {
      // for tkey
      if let Some(tkey) = tkey {
        if !tfield.contains_key(&tkey) {
          tfield.insert(tkey.clone(), vec![]);
        }
        let values = tfield.get_mut(&tkey).unwrap();
        for value in tvalue {
          values.push(value);
        }
        tvalue = vec![];
      }
      tkey = Some(subtag.to_string());
      iter.next();
    } else if (3..=8).contains(&len) && subtag_bytes.iter().all(|c| c.is_ascii_alphanumeric()) {
      // for tvalue
      if tkey.is_none() {
        return Err(ParserError::InvalidSubtag);
      }
      tvalue.push(subtag.to_string());
      iter.next();
    } else if is_language_subtag(subtag_bytes) {
      tlang = Some(parse_unicode_language_id_from_iter(iter)?);
    } else {
      return Err(ParserError::InvalidSubtag);
    }
  }

  if let Some(tkey) = tkey {
    if tvalue.is_empty() {
      return Err(ParserError::InvalidSubtag);
    }
    if !tfield.contains_key(&tkey) {
      tfield.insert(tkey.clone(), vec![]);
    }
    let values = tfield.get_mut(&tkey).unwrap();
    for value in tvalue {
      values.push(value);
    }
  }

  Ok(TransformedExtensions { tlang, tfield })
}

/*
 * Unit tests
 */

#[allow(unused_imports)] // for unit tests
use crate::shared::split_str;

#[test]
fn success_transformed_extensions() {
  // basic case
  let mut iter = split_str("en-US-a1-foo").peekable();
  assert_eq!(
    "t-en-US-a1-foo",
    format!("{}", parse_transformed_extensions(&mut iter).unwrap())
  );

  // no tlang
  let mut iter = split_str("a1-foo").peekable();
  assert_eq!(
    "t-a1-foo",
    format!("{}", parse_transformed_extensions(&mut iter).unwrap())
  );

  // tvalue multiple
  let mut iter = split_str("en-a1-foo-b1-bar").peekable();
  assert_eq!(
    "t-en-a1-foo-b1-bar",
    format!("{}", parse_transformed_extensions(&mut iter).unwrap())
  );

  // tlang only
  let mut iter = split_str("en-Latn-US-macos").peekable();
  assert_eq!(
    "t-en-Latn-US-macos",
    format!("{}", parse_transformed_extensions(&mut iter).unwrap())
  );
}

#[test]
fn fail_transformed_extensions() {
  // invalid tkey
  let mut iter = split_str("1a-foo").peekable();
  assert_eq!(
    ParserError::InvalidSubtag,
    parse_transformed_extensions(&mut iter).unwrap_err()
  );

  // missing tkey
  let mut iter = split_str("foo").peekable();
  assert_eq!(
    ParserError::InvalidSubtag,
    parse_transformed_extensions(&mut iter).unwrap_err()
  );

  // missing tvalue
  let mut iter = split_str("a1-foo-b1").peekable();
  assert_eq!(
    ParserError::InvalidSubtag,
    parse_transformed_extensions(&mut iter).unwrap_err()
  );
}
