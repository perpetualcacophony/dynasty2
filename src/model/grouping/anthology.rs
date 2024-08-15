use std::ops::Deref;

use crate::{
    model::{ChapterMeta, TagType},
    Dynasty,
};

use super::Grouping;

#[derive(serde::Deserialize, Debug, Clone, PartialEq, Eq, Hash)]
pub struct Anthology {
    #[serde(flatten)]
    grouping: Grouping,

    taggings: Vec<ChapterMeta>,
}

impl Anthology {
    pub async fn get(dynasty: &Dynasty, slug: &str) -> crate::Result<Self> {
        dynasty
            .get_json(crate::Path::Tag(TagType::Anthology), slug)
            .await
    }

    pub fn chapters(&self) -> impl Iterator<Item = &ChapterMeta> {
        self.taggings.iter()
    }
}

impl Deref for Anthology {
    type Target = Grouping;

    fn deref(&self) -> &Self::Target {
        &self.grouping
    }
}
