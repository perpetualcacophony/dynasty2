use std::str::FromStr;

use super::{ParseError, Slug};

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct SlugOwned {
    inner: String,
}

impl SlugOwned {
    pub fn as_ref(&self) -> Slug {
        Slug { inner: &self.inner }
    }

    pub fn from_ref(slug: Slug) -> Self {
        Self {
            inner: slug.inner.to_owned(),
        }
    }
}

impl FromStr for SlugOwned {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Slug::parse(s).map(Slug::to_owned)
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
