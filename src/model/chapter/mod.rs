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

#[derive(serde::Deserialize, Clone, Debug)]
pub struct Chapter {
    #[serde(flatten)]
    pub meta: Meta,

    pub long_title: String,

    pub added_on: String,

    pub pages: Vec<Page>,
}

impl Deref for Chapter {
    type Target = Meta;

    fn deref(&self) -> &Self::Target {
        &self.meta
    }
}

impl Chapter {
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
