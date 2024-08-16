use crate::{model::TagMeta, Date, Dynasty, Series, Slug, SlugOwned};

use super::Chapter;

/// Minimal detail about a chapter, included in other responses.
///
/// A full [`Chapter`] response can be requested using [`ChapterMeta::chapter`].
/// `Chapter` dereferences to this type, and gets these methods for free.
#[derive(serde::Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
pub struct ChapterMeta {
    title: String,

    permalink: SlugOwned,

    released_on: Date,

    dynasty_index: Option<usize>,

    tags: Vec<TagMeta>,
}

impl ChapterMeta {
    /// Requests a complete [`Chapter`] using this metadata.
    pub async fn chapter(&self, dynasty: &Dynasty) -> crate::Result<Chapter> {
        Chapter::from_meta(dynasty, self).await
    }

    pub fn title(&self) -> &str {
        &self.title
    }

    pub fn slug(&self) -> Slug {
        self.permalink.as_ref()
    }

    pub fn tags(&self) -> impl Iterator<Item = &TagMeta> {
        self.tags.iter()
    }

    pub fn dynasty_index(&self) -> Option<usize> {
        self.dynasty_index
    }

    pub fn released_on(&self) -> &Date {
        &self.released_on
    }

    pub fn set_dynasty_index(&mut self, index: Option<usize>) {
        self.dynasty_index = index
    }

    pub fn series_tag(&self) -> Option<&TagMeta> {
        self.tags().find(|tag| tag.is_series())
    }

    pub async fn series(&self, dynasty: &Dynasty) -> Option<crate::Result<Series>> {
        if let Some(tag) = self.series_tag() {
            Some(Series::get(dynasty, tag.slug()).await)
        } else {
            None
        }
    }
}
