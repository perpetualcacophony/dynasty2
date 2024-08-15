use crate::model::TagMeta;

#[derive(serde::Deserialize)]
pub struct SeriesJson {
    pub name: String,

    #[serde(rename = "type")]
    pub type_: String,

    pub permalink: String,

    pub cover: String,

    pub link: Option<String>,

    pub description: String,

    pub taggings: Box<[TaggingsItem]>,

    pub tags: Box<[TagMeta]>,
}

#[derive(serde::Deserialize)]
#[serde(untagged)]
pub enum TaggingsItem {
    Header {
        header: String,
    },
    Chapter {
        title: String,
        permalink: String,
        released_on: String,
    },
}
