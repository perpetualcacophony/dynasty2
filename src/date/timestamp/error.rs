use std::{fmt::Display, num::ParseIntError};

use super::TimestampField;

use super::ParseDateError;

#[derive(Debug, PartialEq)]
pub struct ParseTimestampError {
    kind: ParseTimestampErrorKind,
}

impl ParseTimestampError {
    pub fn missing_field(input: &str, field: TimestampField) -> Self {
        Self {
            kind: ParseTimestampErrorKind::MissingField(MissingFieldError {
                field,
                input: input.to_owned().into_boxed_str(),
            }),
        }
    }

    pub fn parse_field(input: &str, inner: ParseIntError, field: TimestampField) -> Self {
        Self {
            kind: ParseTimestampErrorKind::ParseField(ParseFieldError {
                field,
                inner,
                input: input.to_owned().into_boxed_str(),
            }),
        }
    }

    pub fn missing_delimiter(input: &str) -> Self {
        Self {
            kind: ParseTimestampErrorKind::MissingDelimiter(MissingDelimiterError {
                input: input.to_owned().into_boxed_str(),
            }),
        }
    }

    pub fn missing_offset(input: &str) -> Self {
        Self {
            kind: ParseTimestampErrorKind::MissingOffset(MissingOffsetError {
                input: input.to_owned().into_boxed_str(),
            }),
        }
    }
}

impl From<ParseDateError> for ParseTimestampError {
    fn from(value: ParseDateError) -> Self {
        Self {
            kind: ParseTimestampErrorKind::ParseDate(value),
        }
    }
}

impl Display for ParseTimestampError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("failed to parse timestamp")
    }
}

#[derive(Debug, PartialEq)]
pub enum ParseTimestampErrorKind {
    ParseDate(ParseDateError),
    MissingField(MissingFieldError),
    MissingDelimiter(MissingDelimiterError),
    MissingOffset(MissingOffsetError),
    ParseField(ParseFieldError),
}

#[derive(Debug, PartialEq)]
pub struct MissingFieldError {
    field: TimestampField,
    input: Box<str>,
}

#[derive(Debug, PartialEq)]
pub struct ParseFieldError {
    field: TimestampField,
    inner: ParseIntError,
    input: Box<str>,
}

#[derive(Debug, PartialEq)]
pub struct MissingDelimiterError {
    input: Box<str>,
}

#[derive(Debug, PartialEq)]
pub struct MissingOffsetError {
    input: Box<str>,
}
