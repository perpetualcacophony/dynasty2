use std::ops::Deref;

use crate::model::TagMeta;

use super::Kind;

#[derive(serde::Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
pub struct GroupingMeta {
    #[serde(rename = "type")]
    tag: TagMeta<Kind>,

    cover: String,
}

impl GroupingMeta {
    pub fn kind(&self) -> &Kind {
        &self.type_
    }

    pub fn cover(&self) -> &str {
        &self.cover
    }
}

impl Deref for GroupingMeta {
    type Target = TagMeta<Kind>;

    fn deref(&self) -> &Self::Target {
        &self.tag
    }
}
