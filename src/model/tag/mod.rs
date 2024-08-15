mod meta;
pub use meta::TagMeta as Meta;

mod tag_type;
pub use tag_type::TagType as Type;

mod parse;
pub use parse::ParseTag as Parse;

use std::ops::Deref;

use crate::Dynasty;

#[derive(serde::Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
pub struct Tag<Meta = meta::TagMeta> {
    #[serde(flatten)]
    meta: Meta,

    tags: Vec<meta::TagMeta>,

    pub aliases: Vec<String>,

    #[serde(flatten)]
    pub pages: Option<PagesJson>,
}

impl Tag {
    pub async fn get(dynasty: &Dynasty, tag_type: Type, slug: &str) -> crate::Result<Self> {
        dynasty.tag(tag_type, slug).await
    }
}

impl<M> Tag<M> {
    pub fn tags(&self) -> impl Iterator<Item = &Meta> {
        self.tags.iter()
    }
}

impl<Meta> Deref for Tag<Meta> {
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
