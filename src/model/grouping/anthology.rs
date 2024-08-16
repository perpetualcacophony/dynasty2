use std::ops::Deref;

use crate::{
    model::{ChapterMeta, TagType},
    Dynasty, Slug,
};

use super::Inner;

/// A collection of chapters by various authors.
#[derive(serde::Deserialize, Debug, Clone, PartialEq, Eq, Hash)]
pub struct Anthology {
    #[serde(flatten)]
    grouping: Inner,

    taggings: Vec<ChapterMeta>,
}

impl Anthology {
    pub async fn get(dynasty: &Dynasty, slug: &Slug) -> crate::Result<Self> {
        dynasty
            .get_json(crate::Path::Tag(TagType::Anthology), slug)
            .await
    }

    pub fn chapters(&self) -> impl Iterator<Item = &ChapterMeta> {
        self.taggings.iter()
    }
}

impl Deref for Anthology {
    type Target = Inner;

    fn deref(&self) -> &Self::Target {
        &self.grouping
    }
}
