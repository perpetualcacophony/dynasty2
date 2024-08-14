use crate::model::tag::TagType;

#[derive(serde::Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
pub struct ChapterGroupMeta {
    #[serde(rename = "type")]
    type_: TagType,

    name: String,

    permalink: String,

    cover: String,
}
