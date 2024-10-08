use super::Inner;
use crate::{model::ChapterMeta, RequestParams, Slug};
use std::ops::Deref;

/// A collection of chapters by various authors.
#[derive(serde::Deserialize, Debug, Clone, PartialEq, Eq, Hash)]
pub struct Anthology {
    #[serde(flatten)]
    grouping: Inner,

    taggings: Vec<ChapterMeta>,
}

impl Anthology {
    pub fn chapters(&self) -> impl Iterator<Item = &ChapterMeta> {
        self.taggings.iter()
    }
}

impl crate::Response for Anthology {
    const PATH: crate::Path = crate::Path::new("anthologies");
    type Params<'a> = AnthologyParams<'a>;
}

impl Deref for Anthology {
    type Target = Inner;

    fn deref(&self) -> &Self::Target {
        &self.grouping
    }
}

pub struct AnthologyParams<'a> {
    slug: Slug<'a>,
}

impl<'a> RequestParams<'a> for AnthologyParams<'a> {
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
