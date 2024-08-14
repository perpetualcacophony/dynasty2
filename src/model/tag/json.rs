use std::ops::Deref;

use crate::model::{ChapterGroupMeta, ChapterMeta};

use super::TagType;

#[derive(serde::Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
pub struct TagJson {
    #[serde(flatten)]
    pub meta: TagMeta,

    pub tags: Vec<TagMeta>,

    pub cover: Option<String>,

    pub link: Option<String>,

    pub aliases: Vec<String>,

    pub description: Option<String>,

    pub taggings: Vec<ChapterMeta>,

    pub taggables: Option<Vec<ChapterGroupMeta>>,

    #[serde(flatten)]
    pub pages: Option<PagesJson>,
}

impl TagJson {
    pub fn chapters(&self) -> impl Iterator<Item = &ChapterMeta> {
        self.taggings.iter()
    }
}

impl Deref for TagJson {
    type Target = TagMeta;

    fn deref(&self) -> &Self::Target {
        &self.meta
    }
}

#[derive(serde::Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
pub struct PagesJson {
    pub current_page: usize,

    pub total_pages: usize,
}

#[derive(serde::Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
pub struct TagMeta {
    pub name: String,

    #[serde(rename = "type")]
    pub type_: TagType,

    pub permalink: String,
}

impl TagMeta {
    pub fn slug(&self) -> &str {
        &self.permalink
    }

    pub fn is_series(&self) -> bool {
        self.type_ == TagType::Series
    }

    pub fn is_crossover(&self) -> bool {
        self.slug() == "crossover"
    }
}
