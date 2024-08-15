use crate::model::{ChapterMeta, GroupingMeta};

#[derive(serde::Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
pub struct TagView {
    taggings: Option<Vec<ChapterMeta>>,
    taggables: Option<Vec<GroupingMeta>>,
}

pub enum TagViewKind {
    Chapters,
    Groupings,
    OneShots,
}
