use crate::model::{ChapterMeta, GroupingMeta, TagMeta};

#[derive(serde::Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
pub struct Chapters {
    taggings: Vec<ChapterMeta>,
}

impl Chapters {
    pub fn iter(&self) -> impl Iterator<Item = &ChapterMeta> {
        self.taggings.iter()
    }
}

#[derive(serde::Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
pub struct Groupings {
    taggables: Vec<GroupingMeta>,
}

impl Groupings {
    pub fn iter(&self) -> impl Iterator<Item = &GroupingMeta> {
        self.taggables.iter()
    }
}

#[derive(serde::Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
pub struct OneShots {
    taggings: Vec<ChapterMeta>,
}

impl OneShots {
    pub fn iter(&self) -> impl Iterator<Item = &ChapterMeta> {
        self.taggings.iter()
    }
}

#[derive(serde::Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
pub struct Pairings {
    taggables: Vec<TagMeta>,
}

impl Pairings {
    pub fn iter(&self) -> impl Iterator<Item = &TagMeta> {
        self.taggables.iter()
    }
}
