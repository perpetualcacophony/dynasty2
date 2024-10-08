use std::str::FromStr;

use super::{ParseError, Slug};

/// The owned variant of the [`Slug`] type, which can be deserialized.
#[derive(Clone, Debug, PartialEq, Eq, Hash, serde::Serialize)]
#[serde(transparent)]
pub struct SlugOwned {
    inner: String,
}

impl SlugOwned {
    pub fn as_ref(&self) -> Slug {
        Slug { inner: &self.inner }
    }

    pub(super) fn from_ref(slug: Slug) -> Self {
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
        Ok(Self::from_ref(serde::Deserialize::deserialize(
            deserializer,
        )?))
    }
}
