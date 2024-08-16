use std::ops::Deref;

use crate::model::TagMeta;

#[derive(serde::Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
pub struct ChapterJson {
    #[serde(flatten)]
    pub meta: ChapterMeta,

    pub long_title: String,
    pub added_on: String,
    pub pages: Box<[PageJson]>,
}

impl ChapterJson {
    pub fn tags(&self) -> impl Iterator<Item = &TagMeta> {
        self.meta.tags.iter()
    }

    pub fn long_title(&self) -> &str {
        &self.long_title
    }
}

impl Deref for ChapterJson {
    type Target = ChapterMeta;

    fn deref(&self) -> &Self::Target {
        &self.meta
    }
}

#[derive(serde::Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
pub struct PageJson {
    pub name: String,
    pub url: String,
}

#[derive(serde::Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
pub struct ChapterMeta {
    title: String,

    permalink: String,

    pub released_on: String,

    dynasty_index: Option<usize>,

    pub tags: Vec<TagMeta>,
}

impl ChapterMeta {
    pub fn title(&self) -> &str {
        &self.title
    }

    pub fn slug(&self) -> &str {
        &self.permalink
    }

    pub fn tags(&self) -> impl Iterator<Item = &TagMeta> {
        self.tags.iter()
    }

    pub fn dynasty_index(&self) -> Option<usize> {
        self.dynasty_index
    }

    pub fn set_dynasty_index(&mut self, index: Option<usize>) {
        self.dynasty_index = index
    }
}
