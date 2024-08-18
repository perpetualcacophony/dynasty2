use std::ops::Deref;

use super::Inner;

#[derive(serde::Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
pub struct Tag {
    #[serde(flatten)]
    inner: Inner,
}

impl crate::Response for Tag {
    const PATH: crate::Path = crate::Path::new("tags");
}

impl Deref for Tag {
    type Target = Inner;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
