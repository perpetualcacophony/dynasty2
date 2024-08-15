mod json;
use futures::{Stream, StreamExt};
pub use json::ChapterJson as Json;

pub mod index;
pub use index::ChapterIndex as Index;

pub mod id;
pub use id::ChapterId as Id;

use crate::Dynasty;

use super::{Series, Tag, TagMeta};

pub use json::ChapterMeta as Meta;

mod group;
pub use group::{ChapterGroupMeta as GroupMeta, ChapterGroupTagging as GroupTagging};

pub struct Chapter {
    json: Json,
    pub dynasty_index: Option<usize>,
}

impl Chapter {
    pub async fn get(dynasty: &Dynasty, slug: &str) -> crate::Result<Self> {
        Ok(Self {
            json: dynasty.get_json(crate::Path::Chapter, slug).await?,
            dynasty_index: None,
        })
    }

    pub async fn from_meta(dynasty: &Dynasty, meta: &Meta) -> crate::Result<Self> {
        Self::get(dynasty, meta.slug()).await
    }

    pub fn pages(&self) -> impl Iterator<Item = Page> {
        self.json.pages.iter().map(|page| Page {
            filename: &page.name,
            permalink: &page.url,
        })
    }

    pub fn title(&self) -> &str {
        &self.json.meta.title
    }

    pub fn permalink(&self) -> &str {
        &self.json.meta.permalink
    }

    pub fn set_dynasty_index(&mut self, index: usize) {
        self.dynasty_index = Some(index)
    }

    pub fn id(&self) -> Id {
        Id::from_permalink(self.permalink(), self.series_tag().map(TagMeta::slug))
    }

    pub fn index(&self) -> Option<Index> {
        self.id().index()
    }

    pub fn series_tag(&self) -> Option<&TagMeta> {
        self.json.tags().find(|tag| tag.is_series())
    }

    pub async fn series(&self, dynasty: &Dynasty) -> Option<crate::Result<Series>> {
        if let Some(tag) = self.series_tag() {
            Some(dynasty.series(tag.slug()).await)
        } else {
            None
        }
    }

    pub fn tags<'a>(&'a self, dynasty: &'a Dynasty) -> impl Stream<Item = crate::Result<Tag>> + 'a {
        futures::stream::iter(self.json.tags()).then(|tag| tag.get(dynasty))
    }
}

pub struct Page<'ch> {
    pub filename: &'ch str,
    permalink: &'ch str,
}

impl<'ch> Page<'ch> {
    pub fn url(&self) -> String {
        format!(
            "{host}{path}",
            host = crate::Http::HOST_URL,
            path = self.permalink
        )
    }
}
