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
