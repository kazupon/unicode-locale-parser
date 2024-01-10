use crate::constants::{LEGACY_SEP, SEP};

use std::str::Split;

pub fn split_str(s: &str) -> Split<'_, impl Fn(char) -> bool> {
  s.split(|c| c == SEP || c == LEGACY_SEP)
}
