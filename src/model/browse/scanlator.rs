use std::ops::Deref;

use crate::{model::ChapterMeta, Dynasty, Slug};

use super::{Pages, Tag};

/// Represents a single scanlator or scanlation group and their translated chapters.
#[derive(serde::Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
pub struct Scanlator {
    #[serde(flatten)]
    tag: Tag,

    #[serde(flatten)]
    pages: Pages,

    taggings: Vec<ChapterMeta>,
}

impl Deref for Scanlator {
    type Target = Tag;

    fn deref(&self) -> &Self::Target {
        &self.tag
    }
}

impl Scanlator {
    pub async fn get(dynasty: &Dynasty, slug: Slug<'_>) -> crate::Result<Self> {
        <Self as crate::Response>::get(dynasty, slug).await
    }

    pub fn chapters(&self) -> impl Iterator<Item = &ChapterMeta> {
        self.taggings.iter()
    }
}

impl crate::Response for Scanlator {
    const PATH: crate::Path = crate::Path::new("scanlators");
}
