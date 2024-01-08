//! The parser for [Unicode Locale Identifiers](https://unicode.org/reports/tr35/#Unicode_locale_identifier)
//!
//! ## ⛏️ Conformance
//!
//! All code implements of [Unicode UTS #35 Language and Locale Identifiers](https://unicode.org/reports/tr35/#Identifiers).
//!
//! ## 🚀 Usages
//!
//! ```
//! use unicode_locale_parser::parse_locale_id;
//!
//! # fn main() {
//! // simple language
//! let locale = parse_locale_id("ja-JP");
//! println!("{:#?}", locale);
//!
//! // language & unicode locale extension
//! let locale = parse_locale_id("de-Latn-DE-u-ca-buddhist");
//! println!("{:#?}", locale);
//! # }
//! ```
//!
//! ## 🤝 API
//! - [`parse_locale_id`]: parse [`unicode_locale_id`](https://unicode.org/reports/tr35/#unicode_locale_id)
//! - [`parse_language_id`]: parse [`unicode_language_id`](https://unicode.org/reports/tr35/#unicode_language_id)
//! - [`parse_subdivision_id`]: parse [`unicode_subdivision_id`](https://unicode.org/reports/tr35/#unicode_subdivision_id)
//! - [`parse_measure_unit`]: parse [`unicode_measure_unit`](https://unicode.org/reports/tr35/#unicode_measure_unit)
mod constants;
mod extensions;
mod shared;
mod subtags;

mod errors;
mod lang;
mod locale;
mod measure;
mod subdivision;

pub use crate::errors::ParserError;
pub use crate::extensions::other::OtherExtensions;
pub use crate::extensions::pu::PuExtensions;
pub use crate::extensions::transformed::TransformedExtensions;
pub use crate::extensions::unicode_locale::UnicodeLocaleExtensions;
pub use crate::extensions::Extensions;
pub use crate::lang::{parse_unicode_language_id as parse_language_id, UnicodeLanguageIdentifier};
pub use crate::locale::{parse_unicode_locale_id as parse_locale_id, UnicodeLocaleIdentifier};
pub use crate::measure::{parse_unicode_measure_unit as parse_measure_unit, UnicodeMeasureUnit};
pub use crate::subdivision::{
    parse_unicode_subdivision_id as parse_subdivision_id, UnicodeSubdivisionIdentifier,
};
