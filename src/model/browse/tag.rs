use std::ops::Deref;

use crate::Dynasty;

use super::Inner;

#[derive(serde::Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
pub struct Tag {
    #[serde(flatten)]
    inner: Inner,
}

impl Tag {
    pub async fn get(dynasty: &Dynasty, slug: &str) -> crate::Result<Self> {
        dynasty
            .get_json(crate::Path::Tag(crate::model::TagType::General), slug)
            .await
    }
}

impl Deref for Tag {
    type Target = Inner;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
