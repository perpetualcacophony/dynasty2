use std::{fmt::Display, num::ParseIntError, str::FromStr};

use super::{Date, ParseDateError};

#[derive(Copy, Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct Timestamp {
    date: Date,
    hour: u8,
    minute: u8,
    second: u8,
    offset_hour: i8,
    offset_minute: u8,
}

impl Timestamp {
    pub fn new(date: Date, hour: u8, minute: u8, second: u8, offset: (i8, u8)) -> Self {
        Self {
            date,
            hour,
            minute,
            second,
            offset_hour: offset.0,
            offset_minute: offset.1,
        }
    }
}

pub enum ParseTimestampError {
    ParseDate(ParseDateError),
    MissingDelimiter,
    MissingField(MissingFieldError),
    InvalidField(InvalidFieldError),
}

impl Display for ParseTimestampError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}

impl From<ParseDateError> for ParseTimestampError {
    fn from(value: ParseDateError) -> Self {
        Self::ParseDate(value)
    }
}

impl From<MissingFieldError> for ParseTimestampError {
    fn from(value: MissingFieldError) -> Self {
        Self::MissingField(value)
    }
}

impl From<InvalidFieldError> for ParseTimestampError {
    fn from(value: InvalidFieldError) -> Self {
        Self::InvalidField(value)
    }
}

pub enum TimestampField {
    Hour,
    Minute,
    Second,
    Offset,
}

pub struct MissingFieldError {
    field: TimestampField,
}

pub struct InvalidFieldError {
    field: TimestampField,
    err: ParseIntError,
}

impl FromStr for Timestamp {
    type Err = ParseTimestampError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (date, time) = s
            .split_once('T')
            .ok_or(ParseTimestampError::MissingDelimiter)?;

        let date = Date::from_str(date)?;

        let (time, offset) = (&time[0..8], &time[8..]);

        let mut time_split = time.splitn(3, ':');

        let hour = time_split
            .next()
            .ok_or(MissingFieldError {
                field: TimestampField::Hour,
            })?
            .parse()
            .map_err(|err| InvalidFieldError {
                field: TimestampField::Hour,
                err,
            })?;

        let minute = time_split
            .next()
            .ok_or(MissingFieldError {
                field: TimestampField::Minute,
            })?
            .parse()
            .map_err(|err| InvalidFieldError {
                field: TimestampField::Minute,
                err,
            })?;

        let second = time_split
            .next()
            .ok_or(MissingFieldError {
                field: TimestampField::Second,
            })?
            .parse()
            .map_err(|err| InvalidFieldError {
                field: TimestampField::Second,
                err,
            })?;

        let mut offset_split = offset.splitn(2, ':');

        let offset_hour = offset_split
            .next()
            .ok_or(MissingFieldError {
                field: TimestampField::Offset,
            })?
            .parse()
            .map_err(|err| InvalidFieldError {
                field: TimestampField::Offset,
                err,
            })?;

        let offset_minute = offset_split
            .next()
            .ok_or(MissingFieldError {
                field: TimestampField::Offset,
            })?
            .parse()
            .map_err(|err| InvalidFieldError {
                field: TimestampField::Offset,
                err,
            })?;

        Ok(Self::new(
            date,
            hour,
            minute,
            second,
            (offset_hour, offset_minute),
        ))
    }
}

impl<'de> serde::Deserialize<'de> for Timestamp {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = <&str>::deserialize(deserializer)?;
        Self::from_str(s).map_err(serde::de::Error::custom)
    }
}
