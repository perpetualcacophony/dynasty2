use crate::model::{ChapterMeta, GroupingMeta};

#[derive(serde::Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
pub struct BrowseItems<Taggable = GroupingMeta> {
    taggings: Option<Vec<ChapterMeta>>,
    taggables: Option<Vec<Taggable>>,
}
