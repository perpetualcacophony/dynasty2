use std::{fmt::Display, str::FromStr};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
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

        let year = split
            .next()
            .ok_or(ParseError::Year)?
            .parse()
            .map_err(|_| ParseError::Year)?;

        let month = split
            .next()
            .ok_or(ParseError::Month)?
            .parse()
            .map_err(|_| ParseError::Month)?;

        let day = split
            .next()
            .ok_or(ParseError::Day)?
            .parse()
            .map_err(|_| ParseError::Day)?;

        Ok(Self::new(year, month, day))
    }
}

pub enum ParseError {
    Year,
    Month,
    Day,
}

impl Display for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        todo!()
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
