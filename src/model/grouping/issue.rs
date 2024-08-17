use std::ops::Deref;

use crate::{model::ChapterMeta, Dynasty, Slug};

use super::Inner;

#[derive(serde::Deserialize, Debug, Clone, PartialEq, Eq, Hash)]
pub struct Issue {
    #[serde(flatten)]
    grouping: Inner,

    taggings: Vec<ChapterMeta>,
}

impl Issue {
    pub async fn get(dynasty: &Dynasty, slug: Slug<'_>) -> crate::Result<Self> {
        <Self as crate::Response>::get(dynasty, slug).await
    }

    pub fn chapters(&self) -> impl Iterator<Item = &ChapterMeta> {
        self.taggings.iter()
    }
}

impl crate::Response for Issue {
    const PATH: crate::Path = crate::Path::new("issues");
}

impl Deref for Issue {
    type Target = Inner;

    fn deref(&self) -> &Self::Target {
        &self.grouping
    }
}
