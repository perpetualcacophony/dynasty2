use crate::{Path, Slug};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Link<'a> {
    Path(LinkPath<'a>),
    Image(ImageLink),
}

impl Link<'_> {
    pub fn url(&self) -> url::Url {
        crate::Http::host_url()
            .join(self.to_string().as_str())
            .expect("should be a valid url")
    }
}

impl std::fmt::Display for Link<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Path(path) => path.fmt(f),
            Self::Image(image) => image.fmt(f),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct LinkPath<'a> {
    path: Path,
    slug: Slug<'a>,
}

impl<'a> LinkPath<'a> {
    pub fn url(&self) -> url::Url {
        crate::Http::host_url()
            .join(self.to_string().as_str())
            .expect("should be a valid url")
    }

    pub fn new(path: Path, slug: Slug<'a>) -> Self {
        Self { path, slug }
    }
}

impl std::fmt::Display for LinkPath<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use std::fmt::Write;

        self.path.fmt(f)?;
        f.write_char('/')?;
        self.slug.fmt(f)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ImageLink {
    inner: String,
}

impl ImageLink {
    pub fn from_string(s: String) -> Option<Self> {
        if s.starts_with("/system/") {
            Some(Self { inner: s })
        } else {
            None
        }
    }

    pub fn as_str(&self) -> &str {
        &self.inner.as_str()[1..]
    }

    pub fn url(&self) -> url::Url {
        crate::Http::host_url()
            .join(self.as_str())
            .expect("should be a valid url")
    }
}

impl std::fmt::Display for ImageLink {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}

impl<'de> serde::Deserialize<'de> for ImageLink {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        Self::from_string(String::deserialize(deserializer)?)
            .ok_or_else(|| serde::de::Error::custom("image link should start with a slash"))
    }
}
