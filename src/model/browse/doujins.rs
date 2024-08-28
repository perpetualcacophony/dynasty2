use std::ops::Deref;

use crate::{
    model::{GroupingMeta, TagMeta},
    RequestParams, Slug,
};

use super::Inner;

/// Self-published chapters based on an existing property.
#[derive(serde::Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
pub struct Doujins {
    #[serde(flatten)]
    inner: Inner,
}

impl crate::Response for Doujins {
    const PATH: crate::Path = crate::Path::new("doujins");
    type Params<'a> = DoujinParams<'a>;
}

impl Deref for Doujins {
    type Target = Inner;

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

pub struct DoujinParams<'a> {
    slug: Slug<'a>,
}

impl<'a> RequestParams<'a> for DoujinParams<'a> {
    fn url<'url, 'builder>(
        self,
        builder: &'url mut crate::response::UrlBuilder<'builder>,
    ) -> &'url mut crate::response::UrlBuilder<'builder>
    where
        Self: 'builder + Sized,
    {
        builder.slug(self.slug)
    }
}
