use std::ops::Deref;

use crate::model::TagInternal;

use super::Pagination;

#[derive(serde::Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
pub struct BrowseInner<View> {
    #[serde(flatten)]
    tag: TagInternal,

    #[serde(flatten)]
    pages: Pagination,

    #[serde(flatten)]
    view: View,
}

impl<T> Deref for BrowseInner<T> {
    type Target = TagInternal;

    fn deref(&self) -> &Self::Target {
        &self.tag
    }
}

#[cfg(feature = "view")]
impl<V: super::View> BrowseInner<V> {
    fn items(&self) -> impl Iterator<Item = &V::Item> {
        self.view.iter()
    }
}
