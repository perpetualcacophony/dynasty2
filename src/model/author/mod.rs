use std::ops::Deref;

use crate::Dynasty;

use super::{ChapterMeta, Chapters, GroupingMeta, Tag};

#[derive(serde::Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
pub struct Author {
    #[serde(flatten)]
    tag: Tag,

    taggings: Vec<ChapterMeta>,

    taggables: Vec<GroupingMeta>,
}

impl Author {
    pub async fn get(dynasty: &Dynasty, slug: &str) -> crate::Result<Self> {
        dynasty
            .get_json(crate::Path::Tag(crate::model::TagType::Author), slug)
            .await
    }

    pub fn chapters(&self) -> Chapters {
        Chapters::new(&self.taggings)
    }

    pub fn series(&self) -> impl Iterator<Item = &GroupingMeta> {
        self.taggables.iter()
    }
}

impl Deref for Author {
    type Target = Tag;

    fn deref(&self) -> &Self::Target {
        &self.tag
    }
}
