use crate::constants::{LANG_UND, SEP};
use crate::errors::ParserError;
use crate::shared::split_str;
use crate::subtags::{language_subtag, region_subtag, script_subtag, variant_subtag};

use std::fmt::{self, Write};
use std::iter::Peekable;
use std::str::FromStr;

#[derive(Debug, PartialEq)]
pub struct UnicodeLanguageIdentifier {
    pub language: String,
    pub script: Option<String>,
    pub region: Option<String>,
    pub variants: Option<Vec<String>>,
}

pub fn parse_unicode_language_id(chunk: &str) -> Result<UnicodeLanguageIdentifier, ParserError> {
    // check empty
    if chunk.is_empty() {
        return Err(ParserError::Missing);
    }

    parse_unicode_language_id_from_iter(&mut split_str(chunk).peekable())
}

pub fn parse_unicode_language_id_from_iter<'a>(
    iter: &mut Peekable<impl Iterator<Item = &'a str>>,
) -> Result<UnicodeLanguageIdentifier, ParserError> {
    // language subtag
    let language = if let Some(lang) = iter.next() {
        language_subtag(lang)?
    } else {
        return Err(ParserError::Unexpected);
    };
    let language = String::from(language);

    // other subtags
    let mut script = None;
    let mut region = None;
    let mut variants = vec![];
    let mut current = 1;
    while let Some(subtag) = iter.peek() {
        if current == 1 {
            if let Ok(script_subtag) = script_subtag(subtag) {
                script = Some(String::from(script_subtag));
                current = 2;
            } else if let Ok(region_subtag) = region_subtag(subtag) {
                region = Some(String::from(region_subtag));
                current = 3;
            } else if let Ok(variant_subtag) = variant_subtag(subtag) {
                variants.push(String::from(variant_subtag));
                current = 3;
            } else {
                break;
            }
        } else if current == 2 {
            if let Ok(region_subtag) = region_subtag(subtag) {
                region = Some(String::from(region_subtag));
                current = 3;
            } else if let Ok(variant_subtag) = variant_subtag(subtag) {
                variants.push(String::from(variant_subtag));
                current = 3;
            } else {
                break;
            }
        } else if let Ok(variant_subtag) = variant_subtag(subtag) {
            variants.push(String::from(variant_subtag));
        } else {
            break;
        }
        iter.next();
    }

    // normalize variants
    let variants = if variants.is_empty() {
        None
    } else {
        variants.dedup();
        Some(variants)
    };

    Ok(UnicodeLanguageIdentifier {
        language,
        script,
        region,
        variants,
    })
}

impl fmt::Display for UnicodeLanguageIdentifier {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.language.is_empty() {
            f.write_str(LANG_UND)?;
        } else {
            self.language.fmt(f)?;
        }
        if let Some(ref script) = self.script {
            f.write_char(SEP)?;
            script.fmt(f)?;
        }
        if let Some(ref region) = self.region {
            f.write_char(SEP)?;
            region.fmt(f)?;
        }
        if let Some(ref variants) = self.variants {
            for variant in variants.iter() {
                f.write_char(SEP)?;
                variant.fmt(f)?;
            }
        }
        Ok(())
    }
}

impl FromStr for UnicodeLanguageIdentifier {
    type Err = ParserError;

    fn from_str(source: &str) -> Result<Self, Self::Err> {
        parse_unicode_language_id(source)
    }
}

/**
 * Unit tests
 */

#[test]
fn success_parse_unicode_language_id() {
    // full case
    let result = parse_unicode_language_id("en-Latn-US-macos-windows-linux").unwrap();
    assert_eq!(result.language, "en");
    assert_eq!(result.script, Some("Latn".to_string()));
    assert_eq!(result.region, Some("US".to_string()));
    assert_eq!(
        result.variants,
        Some(vec![
            "macos".to_string(),
            "windows".to_string(),
            "linux".to_string()
        ])
    );

    // use sep with underscore
    let result = parse_unicode_language_id("en_Latn_US").unwrap();
    assert_eq!(result.language, "en");
    assert_eq!(result.script, Some("Latn".to_string()));
    assert_eq!(result.region, Some("US".to_string()));

    // language subtag only
    let result = parse_unicode_language_id("en").unwrap();
    assert_eq!(result.language, "en");
    assert_eq!(result.script, None);
    assert_eq!(result.region, None);
    assert_eq!(result.variants, None);

    // language subtag and region subtag
    let result = parse_unicode_language_id("en-US").unwrap();
    assert_eq!(result.language, "en");
    assert_eq!(result.script, None);
    assert_eq!(result.region, Some("US".to_string()));
    assert_eq!(result.variants, None);

    // language subtag and script subtag
    let result = parse_unicode_language_id("en-Latn").unwrap();
    assert_eq!(result.language, "en");
    assert_eq!(result.script, Some("Latn".to_string()));
    assert_eq!(result.region, None);
    assert_eq!(result.variants, None);

    // language subtag and variant subtag
    let result = parse_unicode_language_id("en-macos").unwrap();
    assert_eq!(result.language, "en");
    assert_eq!(result.script, None);
    assert_eq!(result.region, None);
    assert_eq!(result.variants, Some(vec!["macos".to_string()]));

    // language subtag, script subtag and region subtag
    let result = parse_unicode_language_id("en-Latn-US").unwrap();
    assert_eq!(result.language, "en");
    assert_eq!(result.script, Some("Latn".to_string()));
    assert_eq!(result.region, Some("US".to_string()));
    assert_eq!(result.variants, None);

    // language subtag: 'root'
    let result = parse_unicode_language_id("root").unwrap();
    assert_eq!(result.language, "");
    assert_eq!(result.script, None);
    assert_eq!(result.region, None);
    assert_eq!(result.variants, None);

    // include language subtag: 'und'
    let result = parse_unicode_language_id("und-Latn-AT-macos").unwrap();
    assert_eq!(result.language, "");
    assert_eq!(result.script, Some("Latn".to_string()));
    assert_eq!(result.region, Some("AT".to_string()));
    assert_eq!(result.variants, Some(vec!["macos".to_string()]));

    // Display trait implementation
    assert_eq!(
        "en-Latn-US-macos",
        format!("{}", parse_unicode_language_id("en-Latn-US-macos").unwrap())
    );
    assert_eq!(
        "und-Latn-US-macos",
        format!(
            "{}",
            parse_unicode_language_id("und-Latn-US-macos").unwrap()
        )
    );

    // PartialEq trait implementation
    assert_eq!(
        parse_unicode_language_id("en-Latn-US").unwrap(),
        parse_unicode_language_id("en-Latn-US").unwrap()
    );

    // FromStr trait implementation
    let result: UnicodeLanguageIdentifier = "en-Latn-US-macos".parse().unwrap();
    assert_eq!("en", result.language);
    assert_eq!(Some("Latn".to_string()), result.script);
    assert_eq!(Some("US".to_string()), result.region);
    assert_eq!(Some(vec!["macos".to_string()]), result.variants);
    let result: UnicodeLanguageIdentifier = "en-Latn-US".parse().unwrap();
    assert_eq!("en-Latn-US", format!("{}", result));
}

#[test]
fn fail_parse_unicode_language_id() {
    // missing language
    assert_eq!(
        ParserError::Missing,
        parse_unicode_language_id("").unwrap_err()
    );
}
