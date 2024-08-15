use crate::model::{ChapterMeta, GroupingMeta};

#[derive(serde::Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
pub struct BrowseItems {
    taggings: Option<Vec<ChapterMeta>>,
    taggables: Option<Vec<GroupingMeta>>,
}
