use std::ops::Deref;

use crate::model::TagInternal;

use super::Pages;

#[cfg(feature = "view")]
use super::ViewChapters;

#[derive(serde::Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
pub struct BrowseInner {
    #[serde(flatten)]
    tag: TagInternal,

    #[serde(flatten)]
    pages: Pages,
}

impl Deref for BrowseInner {
    type Target = TagInternal;

    fn deref(&self) -> &Self::Target {
        &self.tag
    }
}
