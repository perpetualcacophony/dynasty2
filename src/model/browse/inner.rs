use std::ops::Deref;

use crate::model::{GroupingMeta, TagInternal};

use super::{Items, Pages};

#[derive(serde::Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
pub struct BrowseInner<Taggable = GroupingMeta> {
    #[serde(flatten)]
    tag: TagInternal,

    #[serde(flatten)]
    pages: Pages,

    #[serde(flatten)]
    items: Items<Taggable>,
}

impl<T> Deref for BrowseInner<T> {
    type Target = TagInternal;

    fn deref(&self) -> &Self::Target {
        &self.tag
    }
}
