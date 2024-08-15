use std::ops::Deref;

use crate::model::Tag;

use super::{Items, Pages};

#[derive(serde::Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
pub struct BrowseInner {
    #[serde(flatten)]
    tag: Tag,

    #[serde(flatten)]
    pages: Pages,

    #[serde(flatten)]
    items: Items,
}

impl Deref for BrowseInner {
    type Target = Tag;

    fn deref(&self) -> &Self::Target {
        &self.tag
    }
}
