mod meta;
pub use meta::TagMeta as Meta;

mod tag_type;
pub use tag_type::TagType as Type;

mod parse;
pub use parse::ParseTag as Parse;

use std::ops::Deref;

use crate::{model::ChapterGroupMeta, Dynasty};

#[derive(serde::Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
pub struct Tag {
    #[serde(flatten)]
    pub meta: Meta,

    tags: Vec<Meta>,

    pub aliases: Vec<String>,

    pub taggables: Option<Vec<ChapterGroupMeta>>,

    #[serde(flatten)]
    pub pages: Option<PagesJson>,
}

impl Tag {
    pub async fn get(dynasty: &Dynasty, tag_type: Type, slug: &str) -> crate::Result<Self> {
        dynasty.tag(tag_type, slug).await
    }

    pub fn tags(&self) -> impl Iterator<Item = &Meta> {
        self.tags.iter()
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
