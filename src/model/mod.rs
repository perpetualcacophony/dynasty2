pub mod grouping;
pub use grouping::{Anthology, Grouping, Series};

pub mod chapter;
pub use chapter::{Chapter, GroupMeta as ChapterGroupMeta, Meta as ChapterMeta};

pub mod tag;
pub use tag::{Meta as TagMeta, Parse as ParseTag, Tag, Type as TagType};
