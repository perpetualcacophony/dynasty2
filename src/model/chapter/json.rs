use crate::model::TagMeta;

#[derive(serde::Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
pub struct ChapterJson {
    #[serde(flatten)]
    pub meta: ChapterMeta,

    pub long_title: String,
    pub added_on: String,
    pub pages: Box<[PageJson]>,
}

impl ChapterJson {
    pub fn tags(&self) -> impl Iterator<Item = &TagMeta> {
        self.meta.tags.iter()
    }
}

#[derive(serde::Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
pub struct PageJson {
    pub name: String,
    pub url: String,
}

#[derive(serde::Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
pub struct ChapterMeta {
    pub title: String,

    pub permalink: String,

    pub released_on: String,

    pub tags: Vec<TagMeta>,
}
