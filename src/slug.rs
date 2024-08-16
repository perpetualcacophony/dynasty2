use std::{fmt::Display, ops::Deref, str::FromStr};

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct Slug {
    inner: String,
}

impl Slug {
    pub fn as_str(&self) -> &str {
        self.inner.as_str()
    }

    pub fn from_string(s: String) -> Option<Self> {
        if s.contains('/') {
            None
        } else {
            Some(Self { inner: s })
        }
    }
}

impl Deref for Slug {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        self.as_str()
    }
}

impl<'de> serde::Deserialize<'de> for Slug {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        Self::from_str(<&str>::deserialize(deserializer)?).map_err(serde::de::Error::custom)
    }
}

impl<'a> PartialEq<&'a str> for &'a Slug {
    fn eq(&self, other: &&'a str) -> bool {
        self.as_str().eq(*other)
    }
}

impl Display for Slug {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}

#[derive(Debug)]
pub struct ParseError {
    input: Box<str>,
}

impl std::fmt::Display for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("slug must not contain slashes")
    }
}

impl FromStr for Slug {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::from_string(s.to_owned()).ok_or_else(|| ParseError {
            input: s.to_owned().into_boxed_str(),
        })
    }
}
