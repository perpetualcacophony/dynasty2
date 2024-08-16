use super::Meta;
use crate::model::{TagInternal, TagMeta};
use std::ops::Deref;

#[derive(serde::Deserialize, Debug, Clone, PartialEq, Eq, Hash)]
pub struct GroupingInner {
    #[serde(flatten)]
    pub tag: TagInternal<Meta>,

    pub description: Option<String>,
}

impl GroupingInner {
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
