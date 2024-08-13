use std::str::FromStr;

#[derive(Default)]
pub struct ChapterIndex {
    major: usize,
    minor: Option<usize>,
}

impl ChapterIndex {
    fn from_int(value: usize) -> Self {
        Self {
            major: value,
            minor: None,
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
