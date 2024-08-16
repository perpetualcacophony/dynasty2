use std::ops::Deref;

use crate::{
    model::{ChapterMeta, TagType},
    Dynasty, Slug,
};

use super::Inner;

#[derive(serde::Deserialize, Debug, Clone, PartialEq, Eq, Hash)]
pub struct Issue {
    #[serde(flatten)]
    grouping: Inner,

    taggings: Vec<ChapterMeta>,
}

impl Issue {
    pub async fn get(dynasty: &Dynasty, slug: &Slug) -> crate::Result<Self> {
        dynasty
            .get_json(crate::Path::Tag(TagType::Issue), slug)
            .await
    }

    pub fn chapters(&self) -> impl Iterator<Item = &ChapterMeta> {
        self.taggings.iter()
    }
}

impl Deref for Issue {
    type Target = Inner;

    fn deref(&self) -> &Self::Target {
        &self.grouping
    }
}
