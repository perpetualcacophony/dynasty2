use std::ops::Deref;

pub mod index;
pub use index::ChapterIndex as Index;

pub mod id;
pub use id::ChapterId as Id;

mod meta;
pub use meta::ChapterMeta as Meta;

mod page;
pub use page::Page;

mod request;
pub use request::ChapterParams;
pub use request::RequestChapter as Request;

use crate::Timestamp;

use super::TagMeta;

/// Represents a single manga chapter.
///
/// Dynasty internally refers to chapters as 'taggings.'
///
/// This type dereferences to [`ChapterMeta`](meta::ChapterMeta), which provides methods
/// for basic data access.
#[derive(serde::Deserialize, Clone, Debug)]
pub struct Chapter {
    #[serde(flatten)]
    meta: Meta,

    long_title: String,

    added_on: Timestamp,

    pages: Vec<Page>,
}

impl Deref for Chapter {
    type Target = Meta;

    fn deref(&self) -> &Self::Target {
        &self.meta
    }
}

impl Chapter {
    pub fn pages(&self) -> impl Iterator<Item = &Page> {
        self.pages.iter()
    }

    pub fn added_on(&self) -> &Timestamp {
        &self.added_on
    }

    pub fn long_title(&self) -> &str {
        &self.long_title
    }

    pub fn id(&self) -> Id {
        Id::from_slug(self.slug(), self.series_tag().map(TagMeta::slug))
    }

    pub fn index(&self) -> Option<Index> {
        self.id().index()
    }
}

impl crate::Response for Chapter {
    const PATH: crate::Path = crate::Path::new("chapters");
    type Params<'a> = ChapterParams<'a>;
}
