use std::str::FromStr;

#[derive(Default, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ChapterIndex {
    pub major: usize,
    pub minor: Option<usize>,
}

impl ChapterIndex {
    fn from_int(value: usize) -> Self {
        Self {
            major: value,
            minor: None,
        }
    }

    pub fn to_dynasty(self) -> String {
        if let Some(minor) = self.minor {
            format!("{major}_{minor}", major = self.major)
        } else {
            self.major.to_string()
        }
    }
}

impl FromStr for ChapterIndex {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(if let Some((major, minor)) = s.split_once('/') {
            Self {
                major: major.parse::<usize>().map_err(|parse_int| ParseError {
                    s: s.to_owned(),
                    kind: parse_int.into(),
                })?,
                minor: Some(minor.parse::<usize>().map_err(|parse_int| ParseError {
                    s: s.to_owned(),
                    kind: parse_int.into(),
                })?),
            }
        } else {
            Self::from_int(s.parse::<usize>().map_err(|parse_int| ParseError {
                s: s.to_owned(),
                kind: parse_int.into(),
            })?)
        })
    }
}

impl PartialOrd for ChapterIndex {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for ChapterIndex {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        if self.major == other.major {
            self.minor.cmp(&other.minor)
        } else {
            self.major.cmp(&other.major)
        }
    }
}

#[derive(Debug)]
pub struct ParseError {
    s: String,
    kind: ErrorKind,
}

#[derive(Debug)]
pub enum ErrorKind {
    ParseInt(std::num::ParseIntError),
}

impl From<std::num::ParseIntError> for ErrorKind {
    fn from(value: std::num::ParseIntError) -> Self {
        Self::ParseInt(value)
    }
}
