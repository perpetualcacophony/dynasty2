use std::{num::ParseIntError, str::FromStr};

pub mod timestamp;
pub use timestamp::Timestamp;

pub mod error;
pub use error::ParseDateError as ParseError;

/// Represents a specific date.
///
/// [`Chapter`](crate::Chapter) responses have both a `Date` and a [`Timestamp`].
/// The `Date` represents when the chapter was released by the author,
/// while the `Timestamp` represents the precise time it was uploaded
/// to Dynasty's servers.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Date {
    year: u16,
    month: u8,
    day: u8,
}

impl Date {
    pub fn new(year: u16, month: u8, day: u8) -> Self {
        assert!(month <= 12);
        assert!(day <= 31);

        Self { year, month, day }
    }

    pub fn year(&self) -> u16 {
        self.year
    }

    pub fn month(&self) -> u8 {
        self.month
    }

    pub fn day(&self) -> u8 {
        self.day
    }
}

impl FromStr for Date {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut split = s.splitn(3, '-');

        fn get_field<'a, T: FromStr<Err = ParseIntError>>(
            s: &str,
            split: &mut impl Iterator<Item = &'a str>,
            field: DateField,
        ) -> Result<T, ParseError> {
            split
                .next()
                .ok_or_else(|| ParseError::missing_field(s, field))?
                .parse()
                .map_err(|err| ParseError::parse_field(s, err, field))
        }

        let year = get_field(s, &mut split, DateField::Year)?;
        let month = get_field(s, &mut split, DateField::Month)?;
        let day = get_field(s, &mut split, DateField::Day)?;

        Ok(Self::new(year, month, day))
    }
}

impl<'de> serde::Deserialize<'de> for Date {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = <&str>::deserialize(deserializer)?;
        Self::from_str(s).map_err(serde::de::Error::custom)
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub enum DateField {
    Year,
    Month,
    Day,
}

#[cfg(test)]
mod tests {
    use super::Date;

    mod from_str {
        use crate::date::{DateField, ParseError};

        use super::Date;
        use std::str::FromStr;

        #[test]
        fn valid_date() {
            assert_eq!(Date::from_str("2023-09-07"), Ok(Date::new(2023, 9, 7)))
        }

        #[test]
        fn invalid_date() {
            assert_eq!(
                Date::from_str("2023-09"),
                Err(ParseError::missing_field("2023-09", DateField::Day))
            )
        }
    }
}
