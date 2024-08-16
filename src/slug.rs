use std::ops::Deref;

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
        Self::from_string(String::deserialize(deserializer)?)
            .ok_or_else(|| serde::de::Error::custom("slug should not contain any slashes"))
    }
}
