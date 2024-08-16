use std::ops::Deref;

use crate::model::{GroupingMeta, Tag};

use super::{Items, Pages};

#[derive(serde::Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
pub struct BrowseInner<Taggable = GroupingMeta> {
    #[serde(flatten)]
    tag: Tag,

    #[serde(flatten)]
    pages: Pages,

    #[serde(flatten)]
    items: Items<Taggable>,
}

impl<T> Deref for BrowseInner<T> {
    type Target = Tag;

    fn deref(&self) -> &Self::Target {
        &self.tag
    }
}
