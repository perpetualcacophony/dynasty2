pub mod series;
pub use series::Series;

pub mod chapter;
pub use chapter::{
    Chapter, GroupMeta as ChapterGroupMeta, GroupTagging as ChapterGroupTagging,
    Meta as ChapterMeta,
};

pub mod tag;
pub use tag::{Meta as TagMeta, Parse as ParseTag, Tag, Type as TagType};
