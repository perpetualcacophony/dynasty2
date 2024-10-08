use std::ops::Deref;

use crate::{model::ChapterMeta, RequestParams, Slug};

use super::Inner;

#[derive(serde::Deserialize, Debug, Clone, PartialEq, Eq, Hash)]
pub struct Issue {
    #[serde(flatten)]
    grouping: Inner,

    taggings: Vec<ChapterMeta>,
}

impl Issue {
    pub fn chapters(&self) -> impl Iterator<Item = &ChapterMeta> {
        self.taggings.iter()
    }
}

impl crate::Response for Issue {
    const PATH: crate::Path = crate::Path::new("issues");
    type Params<'a> = IssueParams<'a>;
}

impl Deref for Issue {
    type Target = Inner;

    fn deref(&self) -> &Self::Target {
        &self.grouping
    }
}

pub struct IssueParams<'a> {
    slug: Slug<'a>,
}

impl<'a> RequestParams<'a> for IssueParams<'a> {
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
