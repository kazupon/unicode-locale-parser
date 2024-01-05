use std::str::Split;

pub fn split_str<'a>(s: &'a str) -> Split<'_, impl Fn(char) -> bool> {
    s.split(|c| c == '-' || c == '_')
}
