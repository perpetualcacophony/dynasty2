use crate::model::{ChapterMeta, GroupingMeta};

#[derive(serde::Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
pub struct BrowseItems<Taggable = GroupingMeta> {
    taggings: Option<Vec<ChapterMeta>>,
    taggables: Option<Vec<Taggable>>,
}

impl<T> BrowseItems<T> {
    pub fn taggings(&self) -> impl Iterator<Item = &ChapterMeta> {
        self.taggings.iter().flatten()
    }

    pub fn taggables(&self) -> impl Iterator<Item = &T> {
        self.taggables.iter().flatten()
    }
}
