use std::ops::Deref;

use crate::{model::TagInternal, RequestParams, Slug};

#[derive(serde::Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
pub struct Tag {
    #[serde(flatten)]
    inner: TagInternal,
}

impl crate::Response for Tag {
    const PATH: crate::Path = crate::Path::new("tags");
    type Params<'a> = TagParams<'a>;
}

impl Deref for Tag {
    type Target = TagInternal;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

pub struct TagParams<'a> {
    slug: Slug<'a>,
}

impl<'a> RequestParams<'a> for TagParams<'a> {
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
