use serde::de::DeserializeOwned;

use crate::model::{ChapterMeta, GroupingMeta, TagMeta};

#[derive(serde::Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
pub struct Chapters {
    taggings: Vec<ChapterMeta>,
}

impl View for Chapters {
    const ID: &str = "chapters";
    type Item = ChapterMeta;

    fn iter(&self) -> impl Iterator<Item = &Self::Item> {
        self.taggings.iter()
    }
}

#[derive(serde::Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
pub struct Groupings {
    taggables: Vec<GroupingMeta>,
}

impl View for Groupings {
    const ID: &str = "groupings";
    type Item = GroupingMeta;

    fn iter(&self) -> impl Iterator<Item = &Self::Item> {
        self.taggables.iter()
    }
}

#[derive(serde::Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
pub struct OneShots {
    taggings: Vec<ChapterMeta>,
}

impl View for OneShots {
    const ID: &str = "one_shots";
    type Item = ChapterMeta;

    fn iter(&self) -> impl Iterator<Item = &Self::Item> {
        self.taggings.iter()
    }
}

#[derive(serde::Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
pub struct Pairings {
    taggables: Vec<TagMeta>,
}

impl View for Pairings {
    const ID: &str = "pairings";
    type Item = TagMeta;

    fn iter(&self) -> impl Iterator<Item = &Self::Item> {
        self.taggables.iter()
    }
}

pub enum ViewTagging {
    Chapter(ChapterMeta),
    OneShot(ChapterMeta),
}

pub enum ViewTaggable {
    Grouping(GroupingMeta),
    Pairing(TagMeta),
}

pub trait HasView<View> {}

pub trait View: DeserializeOwned {
    const ID: &str;
    type Item;

    fn iter(&self) -> impl Iterator<Item = &Self::Item>;
}
