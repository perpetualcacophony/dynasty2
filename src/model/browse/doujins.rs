use std::ops::Deref;

use crate::{
    model::{GroupingMeta, TagMeta},
    Dynasty, Slug,
};

use super::Inner;

/// Self-published chapters based on an existing property.
#[derive(serde::Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
pub struct Doujins {
    #[serde(flatten)]
    inner: Inner<DoujinTaggable>,
}

impl Doujins {
    pub async fn get(dynasty: &Dynasty, slug: &Slug) -> crate::Result<Self> {
        dynasty
            .get_json(crate::Path::Tag(crate::model::TagType::Doujin), slug)
            .await
    }
}

impl Deref for Doujins {
    type Target = Inner<DoujinTaggable>;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

#[derive(serde::Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
#[serde(untagged)]
pub enum DoujinTaggable {
    Grouping(GroupingMeta),
    Pairing(TagMeta),
}
