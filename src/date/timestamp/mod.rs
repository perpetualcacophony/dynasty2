use std::{num::ParseIntError, str::FromStr};

use super::{Date, ParseError as ParseDateError};

pub mod error;
pub use error::ParseTimestampError as ParseError;

/// Represents a specific time on a given date with second precision.
///
/// Dynasty timestamps have a constant UTC offset of `-04:00`, which is omitted
/// from this struct for consistency.
#[derive(Copy, Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct Timestamp {
    date: Date,
    hour: u8,
    minute: u8,
    second: u8,
}

impl Timestamp {
    const OFFSET_STR: &str = "-04:00";
    const OFFSET_SECS: u16 = 4 * 60 * 60;

    pub fn new(date: Date, hour: u8, minute: u8, second: u8) -> Self {
        Self {
            date,
            hour,
            minute,
            second,
        }
    }

    pub fn hour(&self) -> u8 {
        self.hour
    }

    pub fn minute(&self) -> u8 {
        self.minute
    }

    pub fn second(&self) -> u8 {
        self.second
    }

    #[cfg(feature = "time")]
    pub fn time(self) -> time::OffsetDateTime {
        let time = time::Time::from_hms(self.hour(), self.minute(), self.second())
            .expect("should be valid time");
        let offset = time::UtcOffset::from_hms(-4, 0, 0).expect("should be a valid offset");

        time::OffsetDateTime::new_in_offset(self.date.time(), time, offset)
    }

    #[cfg(feature = "chrono")]
    pub fn chrono(self) -> chrono::DateTime<chrono::FixedOffset> {
        let time = chrono::NaiveTime::from_hms_opt(
            self.hour().into(),
            self.minute().into(),
            self.second().into(),
        )
        .expect("should be valid time");

        let naive = chrono::NaiveDateTime::new(self.date.chrono(), time);

        let offset = chrono::FixedOffset::west_opt(Self::OFFSET_SECS.into())
            .expect("offset should be valid");

        chrono::DateTime::from_naive_utc_and_offset(naive, offset)
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub enum TimestampField {
    Hour,
    Minute,
    Second,
}

impl FromStr for Timestamp {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (date, time) = s.split_once('T').ok_or(ParseError::missing_delimiter(s))?;

        let date = Date::from_str(date)?;

        let time = time
            .strip_suffix(Self::OFFSET_STR)
            .ok_or_else(|| ParseError::missing_offset(s))?;

        let mut split = time.splitn(3, ':');

        fn get_field<'a, T: FromStr<Err = ParseIntError>>(
            s: &str,
            split: &mut impl Iterator<Item = &'a str>,
            field: TimestampField,
        ) -> Result<T, ParseError> {
            split
                .next()
                .ok_or_else(|| ParseError::missing_field(s, field))?
                .parse()
                .map_err(|err| ParseError::parse_field(s, err, field))
        }

        let hour = get_field(s, &mut split, TimestampField::Hour)?;
        let minute = get_field(s, &mut split, TimestampField::Minute)?;
        let second = get_field(s, &mut split, TimestampField::Second)?;

        Ok(Self::new(date, hour, minute, second))
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

#[cfg(test)]
mod tests {
    use super::Timestamp;

    mod from_str {
        use crate::{date::timestamp::TimestampField, Date};

        use super::super::ParseError;
        use super::Timestamp;
        use std::str::FromStr;

        #[test]
        fn valid() {
            assert_eq!(
                Timestamp::from_str("2023-09-07T00:42:14-04:00"),
                Ok(Timestamp::new(Date::new(2023, 9, 7), 0, 42, 14))
            )
        }

        #[test]
        fn invalid() {
            assert_eq!(
                Timestamp::from_str("2023-09-07T00:42-04:00"),
                Err(ParseError::missing_field(
                    "2023-09-07T00:42-04:00",
                    TimestampField::Second
                ))
            )
        }
    }
}
