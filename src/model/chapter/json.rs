use crate::model::Tag;

#[derive(serde::Deserialize)]
pub struct ChapterJson {
    pub title: String,
    pub long_title: String,
    pub permalink: String,
    pub released_on: String,
    pub added_on: String,
    pub pages: Box<[PageJson]>,
    pub tags: Box<[Tag]>,
}

#[derive(serde::Deserialize)]
pub struct PageJson {
    pub name: String,
    pub url: String,
}
