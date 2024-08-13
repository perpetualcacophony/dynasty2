use super::TagType;

#[derive(serde::Deserialize, Clone, Debug, PartialEq, Eq)]
pub struct TagJson {
    #[serde(rename = "type")]
    pub type_: TagType,

    pub name: String,

    pub permalink: String,
}
