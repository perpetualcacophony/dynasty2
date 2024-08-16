use std::ops::Deref;

use crate::model::{ChapterMeta, Chapters};

use super::{Pages, Tag};

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
    pub fn chapters(&self) -> Chapters {
        Chapters::new(&self.taggings)
    }
}
