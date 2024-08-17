use super::Inner;
use crate::{model::ChapterMeta, Dynasty, Slug};
use std::ops::Deref;

/// A collection of chapters by various authors.
#[derive(serde::Deserialize, Debug, Clone, PartialEq, Eq, Hash)]
pub struct Anthology {
    #[serde(flatten)]
    grouping: Inner,

    taggings: Vec<ChapterMeta>,
}

impl Anthology {
    pub async fn get(dynasty: &Dynasty, slug: Slug<'_>) -> crate::Result<Self> {
        <Self as crate::Response>::get(dynasty, slug).await
    }

    pub fn chapters(&self) -> impl Iterator<Item = &ChapterMeta> {
        self.taggings.iter()
    }
}

impl crate::Response for Anthology {
    const PATH: crate::Path = crate::Path::new("anthologies");
}

impl Deref for Anthology {
    type Target = Inner;

    fn deref(&self) -> &Self::Target {
        &self.grouping
    }
}
