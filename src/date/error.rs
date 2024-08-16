use std::{fmt::Display, num::ParseIntError};

use super::DateField;

#[derive(Debug, PartialEq)]
pub struct ParseDateError {
    pub(super) input: Box<str>,
    kind: ParseDateErrorKind,
}

impl ParseDateError {
    pub fn missing_field(input: &str, field: DateField) -> Self {
        Self {
            input: input.to_owned().into_boxed_str(),
            kind: ParseDateErrorKind::MissingField(MissingFieldError { field }),
        }
    }

    pub fn parse_field(input: &str, inner: ParseIntError, field: DateField) -> Self {
        Self {
            input: input.to_owned().into_boxed_str(),
            kind: ParseDateErrorKind::ParseField(ParseFieldError { field, inner }),
        }
    }
}

impl Display for ParseDateError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}

#[derive(Debug, PartialEq)]
pub enum ParseDateErrorKind {
    MissingField(MissingFieldError),
    ParseField(ParseFieldError),
}

#[derive(Debug, PartialEq)]
pub struct MissingFieldError {
    field: DateField,
}

#[derive(Debug, PartialEq)]
pub struct ParseFieldError {
    field: DateField,
    inner: ParseIntError,
}
