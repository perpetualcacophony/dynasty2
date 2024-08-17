use std::ops::Deref;

use crate::{Dynasty, Slug};

use super::{ChapterMeta, GroupingMeta, TagInternal};

/// Represents the creator of select chapters and narratives.
#[derive(serde::Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
pub struct Author {
    #[serde(flatten)]
    tag: TagInternal,

    taggings: Vec<ChapterMeta>,

    taggables: Vec<GroupingMeta>,
}

impl Author {
    pub async fn get(dynasty: &Dynasty, slug: Slug<'_>) -> crate::Result<Self> {
        <Self as crate::Response>::get(dynasty, slug).await
    }

    pub fn chapters(&self) -> impl Iterator<Item = &ChapterMeta> {
        self.taggings.iter()
    }

    pub fn series(&self) -> impl Iterator<Item = &GroupingMeta> {
        self.taggables.iter()
    }
}

impl crate::Response for Author {
    const PATH: crate::Path = crate::Path::new("authors");
}

impl Deref for Author {
    type Target = TagInternal;

    fn deref(&self) -> &Self::Target {
        &self.tag
    }
}
