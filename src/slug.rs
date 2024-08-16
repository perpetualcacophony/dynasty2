use std::{fmt::Display, str::FromStr};

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct SlugOwned {
    inner: String,
}

impl SlugOwned {
    pub fn as_ref(&self) -> Slug {
        Slug { inner: &self.inner }
    }
}

impl FromStr for SlugOwned {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Slug::from_str(s).map(Slug::to_owned)
    }
}

impl<'de> serde::Deserialize<'de> for SlugOwned {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        Self::from_str(<&str>::deserialize(deserializer)?).map_err(serde::de::Error::custom)
    }
}

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
pub struct Slug<'a> {
    inner: &'a str,
}

impl<'a> Slug<'a> {
    pub fn as_str(&self) -> &'a str {
        self.inner
    }

    #[allow(clippy::should_implement_trait)]
    pub fn from_str(s: &'a str) -> Result<Self, ParseError> {
        Self::try_from(s).map_err(|s| ParseError {
            input: s.to_owned().into_boxed_str(),
        })
    }

    pub fn to_owned(self) -> SlugOwned {
        SlugOwned {
            inner: self.as_str().to_owned(),
        }
    }
}

impl<'a> TryFrom<&'a str> for Slug<'a> {
    type Error = &'a str;

    fn try_from(value: &'a str) -> Result<Self, Self::Error> {
        if !value.contains('/') {
            Ok(Self { inner: value })
        } else {
            Err(value)
        }
    }
}

impl Display for Slug<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}

impl<'a> PartialEq<&'a str> for Slug<'a> {
    fn eq(&self, other: &&'a str) -> bool {
        self.as_str().eq(*other)
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
