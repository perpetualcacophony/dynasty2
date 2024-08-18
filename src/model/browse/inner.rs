use std::ops::Deref;

use crate::model::TagInternal;

use super::{Pages, ViewChapters};

#[derive(serde::Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
pub struct BrowseInner<View = ViewChapters> {
    #[serde(flatten)]
    tag: TagInternal,

    #[serde(flatten)]
    pages: Pages,

    #[serde(flatten)]
    view: View,
}

impl<T> Deref for BrowseInner<T> {
    type Target = TagInternal;

    fn deref(&self) -> &Self::Target {
        &self.tag
    }
}
