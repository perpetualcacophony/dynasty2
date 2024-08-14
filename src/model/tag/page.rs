use crate::model::{ChapterGroupMeta, ChapterMeta};

use super::TagType;

#[derive(serde::Deserialize)]
pub struct TagPage {
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

impl TagPage {
    pub fn chapters(&self) -> impl Iterator<Item = &ChapterMeta> {
        self.taggings.iter()
    }
}

#[derive(serde::Deserialize)]
pub struct PagesJson {
    pub current_page: usize,

    pub total_pages: usize,
}

#[derive(serde::Deserialize)]
pub struct TagMeta {
    pub name: String,

    #[serde(rename = "type")]
    pub type_: TagType,

    pub permalink: String,
}
