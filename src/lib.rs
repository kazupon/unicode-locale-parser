#![warn(rustdoc::missing_crate_level_docs)]

mod constants;
mod extensions;
mod shared;
mod subtags;

mod errors;
mod lang;
mod locale;
mod subdivision;

pub use crate::errors::ParserError;
pub use crate::extensions::other::OtherExtensions;
pub use crate::extensions::pu::PuExtensions;
pub use crate::extensions::transformed::TransformedExtensions;
pub use crate::extensions::unicode_locale::UnicodeLocaleExtensions;
pub use crate::extensions::Extensions;
pub use crate::lang::{parse_unicode_language_id, UnicodeLanguageIdentifier};
pub use crate::locale::{parse_unicode_locale_id, UnicodeLocaleIdentifier};
pub use crate::subdivision::{parse_unicode_subdivision_id, UnicodeSubdivisionIdentifier};
