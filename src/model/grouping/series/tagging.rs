use crate::model::ChapterMeta;

/// Represents either a volume header or metadata for a chapter.
///
/// The Dynasty API returns volume headers and chapter metadata
/// in the same array, requiring this wrapper type.
#[derive(serde::Deserialize, Clone, Debug, Hash, PartialEq, Eq)]
#[serde(untagged)]
pub enum SeriesTagging {
    Header(TaggingHeader),
    Chapter(ChapterMeta),
}

impl SeriesTagging {
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
