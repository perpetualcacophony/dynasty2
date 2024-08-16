use std::ops::Deref;

use crate::{
    model::{ChapterMeta, Chapters, TagType},
    Dynasty,
};

use super::Inner;

#[derive(serde::Deserialize, Debug, Clone, PartialEq, Eq, Hash)]
pub struct Anthology {
    #[serde(flatten)]
    grouping: Inner,

    taggings: Vec<ChapterMeta>,
}

impl Anthology {
    pub async fn get(dynasty: &Dynasty, slug: &str) -> crate::Result<Self> {
        dynasty
            .get_json(crate::Path::Tag(TagType::Anthology), slug)
            .await
    }

    pub fn chapters(&self) -> Chapters {
        Chapters::new(&self.taggings)
    }
}

impl Deref for Anthology {
    type Target = Inner;

    fn deref(&self) -> &Self::Target {
        &self.grouping
    }
}
