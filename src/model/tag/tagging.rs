use crate::model::ChapterMeta;

#[derive(serde::Deserialize, Clone, Debug, Hash, PartialEq, Eq)]
#[serde(untagged)]
pub enum Tagging {
    Header(TaggingHeader),
    Chapter(ChapterMeta),
}

impl Tagging {
    pub fn chapter(&self) -> Option<&ChapterMeta> {
        if let Self::Chapter(meta) = self {
            Some(meta)
        } else {
            None
        }
    }
}

#[derive(serde::Deserialize, Clone, Debug, Hash, PartialEq, Eq)]
pub struct TaggingHeader {
    header: String,
}
