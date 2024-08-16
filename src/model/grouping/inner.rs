use std::ops::Deref;

use crate::model::{TagInternal, TagMeta};
use crate::Dynasty;

use super::{Kind, Meta};

#[derive(serde::Deserialize, Debug, Clone, PartialEq, Eq, Hash)]
pub struct GroupingInner {
    #[serde(flatten)]
    pub tag: TagInternal<Meta>,

    pub cover: String,

    pub description: Option<String>,
}

impl GroupingInner {
    pub async fn get(dynasty: &Dynasty, kind: Kind, slug: &str) -> crate::Result<Self> {
        dynasty.get_json(crate::Path::Tag(kind.into()), slug).await
    }

    pub fn title(&self) -> &str {
        self.name()
    }

    pub fn tags(&self) -> impl Iterator<Item = &TagMeta> {
        self.tag.tags()
    }
}

impl Deref for GroupingInner {
    type Target = TagInternal<Meta>;

    fn deref(&self) -> &Self::Target {
        &self.tag
    }
}
