use std::ops::Deref;

use crate::{model::TagMeta, ImageLink};

use super::Kind;

#[derive(serde::Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
pub struct GroupingMeta {
    #[serde(rename = "type", flatten)]
    tag: TagMeta<Kind>,

    cover: ImageLink,
}

impl GroupingMeta {
    pub fn kind(&self) -> &Kind {
        &self.type_
    }

    pub fn cover(&self) -> url::Url {
        self.cover.url()
    }
}

impl Deref for GroupingMeta {
    type Target = TagMeta<Kind>;

    fn deref(&self) -> &Self::Target {
        &self.tag
    }
}
