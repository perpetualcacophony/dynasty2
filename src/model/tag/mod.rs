mod meta;
pub use meta::TagMeta as Meta;

mod tag_type;
pub use tag_type::TagType as Type;

mod parse;
pub use parse::ParseTag as Parse;

mod view;
pub use view::TagView as View;

use std::ops::Deref;

use crate::Dynasty;

#[derive(serde::Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
pub struct Tag<Meta = meta::TagMeta> {
    #[serde(flatten)]
    meta: Meta,

    tags: Vec<meta::TagMeta>,

    aliases: Vec<String>,
}

impl Tag {
    pub async fn get(dynasty: &Dynasty, tag_type: Type, slug: &str) -> crate::Result<Self> {
        dynasty.get_json(crate::Path::Tag(tag_type), slug).await
    }
}

impl<M> Tag<M> {
    pub fn tags(&self) -> impl Iterator<Item = &Meta> {
        self.tags.iter()
    }

    pub fn aliases(&self) -> impl Iterator<Item = &str> {
        self.aliases.iter().map(String::as_str)
    }
}

impl<Meta> Deref for Tag<Meta> {
    type Target = Meta;

    fn deref(&self) -> &Self::Target {
        &self.meta
    }
}
