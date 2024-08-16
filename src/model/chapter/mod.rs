use std::ops::Deref;

pub mod index;
pub use index::ChapterIndex as Index;

pub mod id;
pub use id::ChapterId as Id;

mod meta;
pub use meta::ChapterMeta as Meta;

mod page;
pub use page::Page;

use crate::Dynasty;

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

    added_on: String,

    pages: Vec<Page>,
}

impl Deref for Chapter {
    type Target = Meta;

    fn deref(&self) -> &Self::Target {
        &self.meta
    }
}

impl Chapter {
    /// Requests a `Chapter` with the given slug.
    ///
    /// This method is semantically equivalent to
    /// `https://dynasty-scans.com/chapter/{slug}`
    pub async fn get(dynasty: &Dynasty, slug: &str) -> crate::Result<Self> {
        dynasty.get_json(crate::Path::Chapter, slug).await
    }

    pub async fn from_meta(dynasty: &Dynasty, meta: &Meta) -> crate::Result<Self> {
        let mut new = Self::get(dynasty, meta.slug()).await?;

        new.meta.set_dynasty_index(meta.dynasty_index());

        Ok(new)
    }

    pub fn pages(&self) -> impl Iterator<Item = &Page> {
        self.pages.iter()
    }

    pub fn long_title(&self) -> &str {
        &self.long_title
    }

    pub fn id(&self) -> Id {
        Id::from_permalink(self.slug(), self.series_tag().map(TagMeta::slug))
    }

    pub fn index(&self) -> Option<Index> {
        self.id().index()
    }
}
