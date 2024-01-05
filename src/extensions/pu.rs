use crate::errors::ParserError;
use std::{iter::Peekable, vec};

#[derive(Debug)]
pub struct PuExtensions {
    pub values: Vec<String>,
}

pub fn parse_pu_extensions<'a>(
    iter: &mut Peekable<impl Iterator<Item = &'a str>>,
) -> Result<PuExtensions, ParserError> {
    let mut values = vec![];

    Ok(PuExtensions { values })
}
