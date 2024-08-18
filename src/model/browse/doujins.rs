use std::ops::Deref;

use crate::model::{GroupingMeta, TagMeta};

use super::Inner;

/// Self-published chapters based on an existing property.
#[derive(serde::Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
pub struct Doujins {
    #[serde(flatten)]
    inner: Inner<DoujinTaggable>,
}

impl crate::Response for Doujins {
    const PATH: crate::Path = crate::Path::new("doujins");
}

impl Deref for Doujins {
    type Target = Inner<DoujinTaggable>;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

#[derive(serde::Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
#[serde(untagged)]
pub enum DoujinTaggable {
    Grouping(GroupingMeta),
    Pairing(TagMeta),
}
