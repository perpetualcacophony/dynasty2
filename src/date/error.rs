use std::{fmt::Display, num::ParseIntError};

use super::DateField;

#[derive(Debug, PartialEq)]
pub struct ParseDateError {
    input: Box<str>,
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
        f.write_str("failed to parse date: ")?;

        match &self.kind {
            ParseDateErrorKind::MissingField(err) => err.fmt(f),
            ParseDateErrorKind::ParseField(err) => err.fmt(f),
        }
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

impl Display for MissingFieldError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "missing field `{:?}`", self.field)
    }
}

impl std::error::Error for MissingFieldError {}

#[derive(Debug, PartialEq)]
pub struct ParseFieldError {
    field: DateField,
    inner: ParseIntError,
}

impl Display for ParseFieldError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "failed to parse field field `{:?}`", self.field)
    }
}

impl std::error::Error for ParseFieldError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        Some(&self.inner)
    }
}
