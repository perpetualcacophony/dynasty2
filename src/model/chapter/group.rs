use crate::model::TagType;

#[derive(serde::Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
pub struct ChapterGroupMeta {
    #[serde(rename = "type")]
    type_: TagType,

    name: String,

    permalink: String,

    cover: String,
}

use crate::model::ChapterMeta;

#[derive(serde::Deserialize, Clone, Debug, Hash, PartialEq, Eq)]
#[serde(untagged)]
pub enum ChapterGroupTagging {
    Header(TaggingHeader),
    Chapter(ChapterMeta),
}

impl ChapterGroupTagging {
    pub fn chapter(&self) -> Option<&ChapterMeta> {
        if let Self::Chapter(meta) = self {
            Some(meta)
        } else {
            None
        }
    }

    pub fn header(&self) -> Option<&str> {
        if let Self::Header(header) = self {
            Some(&header.header)
        } else {
            None
        }
    }
}

#[derive(serde::Deserialize, Clone, Debug, Hash, PartialEq, Eq)]
pub struct TaggingHeader {
    header: String,
}
