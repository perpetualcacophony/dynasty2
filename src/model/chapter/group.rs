use crate::model::tag::TagType;

#[derive(serde::Deserialize)]
pub struct ChapterGroupMeta {
    #[serde(rename = "type")]
    type_: TagType,

    name: String,

    permalink: String,

    cover: String,
}
