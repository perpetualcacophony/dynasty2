mod meta;
pub use meta::TagMeta as Meta;

mod tag_type;
pub use tag_type::TagType as Type;

mod parse;
pub use parse::ParseTag as Parse;

mod tagging;
pub use tagging::Tagging;

use std::ops::Deref;

use crate::{
    model::{ChapterGroupMeta, ChapterMeta},
    Dynasty,
};

#[derive(serde::Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
pub struct Tag {
    #[serde(flatten)]
    pub meta: Meta,

    pub tags: Vec<Meta>,

    pub cover: Option<String>,

    pub link: Option<String>,

    pub aliases: Vec<String>,

    pub description: Option<String>,

    pub taggings: Vec<Tagging>,

    pub taggables: Option<Vec<ChapterGroupMeta>>,

    #[serde(flatten)]
    pub pages: Option<PagesJson>,
}

impl Tag {
    pub async fn get(dynasty: &Dynasty, tag_type: Type, slug: &str) -> crate::Result<Self> {
        dynasty.tag(tag_type, slug).await
    }

    pub fn chapters(&self) -> impl Iterator<Item = &ChapterMeta> {
        self.taggings.iter().filter_map(Tagging::chapter)
    }
}

impl Deref for Tag {
    type Target = Meta;

    fn deref(&self) -> &Self::Target {
        &self.meta
    }
}

#[derive(serde::Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
pub struct PagesJson {
    pub current_page: usize,

    pub total_pages: usize,
}
