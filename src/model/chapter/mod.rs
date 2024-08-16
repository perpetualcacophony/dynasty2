mod json;
use futures::{Stream, StreamExt};
pub use json::ChapterJson as Json;

pub mod index;
pub use index::ChapterIndex as Index;

pub mod id;
pub use id::ChapterId as Id;

use crate::Dynasty;

use super::{Series, TagInternal, TagMeta};

pub use json::ChapterMeta as Meta;

#[derive(serde::Deserialize, Clone, Debug)]
pub struct Chapter {
    #[serde(flatten)]
    json: Json,
}

impl Chapter {
    pub async fn get(dynasty: &Dynasty, slug: &str) -> crate::Result<Self> {
        Ok(Self {
            json: dynasty.get_json(crate::Path::Chapter, slug).await?,
        })
    }

    pub async fn from_meta(dynasty: &Dynasty, meta: &Meta) -> crate::Result<Self> {
        let mut new = Self::get(dynasty, meta.slug()).await?;

        new.json.meta.set_dynasty_index(meta.dynasty_index());

        Ok(new)
    }

    pub fn pages(&self) -> impl Iterator<Item = Page> {
        self.json.pages.iter().map(|page| Page {
            filename: &page.name,
            permalink: &page.url,
        })
    }

    pub fn title(&self) -> &str {
        self.json.title()
    }

    pub fn long_title(&self) -> &str {
        self.json.long_title()
    }

    pub fn permalink(&self) -> &str {
        self.json.slug()
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

    pub fn tags<'a>(
        &'a self,
        dynasty: &'a Dynasty,
    ) -> impl Stream<Item = crate::Result<TagInternal>> + 'a {
        futures::stream::iter(self.json.tags()).then(|tag| tag.get(dynasty))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
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

pub struct Chapters<'a, It = std::slice::Iter<'a, Meta>> {
    iter: It,
    phantom: std::marker::PhantomData<&'a ()>,
}

impl<'a, It> Chapters<'a, It>
where
    It: Iterator<Item = &'a Meta> + 'a,
{
    pub fn new(iter: impl IntoIterator<IntoIter = It>) -> Self {
        Self {
            iter: iter.into_iter(),
            phantom: std::marker::PhantomData,
        }
    }

    pub fn meta(self) -> It {
        self.iter
    }

    pub fn stream(self, dynasty: &'a Dynasty) -> impl Stream<Item = crate::Result<Chapter>> + 'a {
        futures::stream::iter(self).then(|meta| Chapter::from_meta(dynasty, meta))
    }
}

impl<'a, It> IntoIterator for Chapters<'a, It>
where
    It: Iterator<Item = &'a Meta> + 'a,
{
    type Item = &'a Meta;
    type IntoIter = It;

    fn into_iter(self) -> Self::IntoIter {
        self.meta()
    }
}
