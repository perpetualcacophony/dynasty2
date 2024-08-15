use futures::{StreamExt, TryStreamExt};

use crate::Dynasty;

use super::{Chapter, ChapterGroupTagging, ChapterMeta, Tag, TagMeta};

#[derive(serde::Deserialize, Debug, Clone, PartialEq, Eq, Hash)]
pub struct Series {
    #[serde(flatten)]
    pub tag: Tag,

    pub cover: String,

    pub description: Option<String>,

    taggings: Vec<ChapterGroupTagging>,
}

impl Series {
    pub async fn get(dynasty: &Dynasty, slug: &str) -> crate::Result<Self> {
        dynasty
            .get_json(crate::Path::Tag(super::TagType::Series), slug)
            .await
    }

    pub fn title(&self) -> &str {
        self.tag.name()
    }

    pub fn cover(&self) -> &str {
        &self.cover
    }

    pub fn tags(&self) -> impl Iterator<Item = &TagMeta> {
        self.tag.tags()
    }

    pub fn taggings(&self) -> impl Iterator<Item = &ChapterGroupTagging> {
        self.taggings.iter()
    }

    pub fn volumes(&self) -> Vec<Volume> {
        let mut counters = Vec::new();

        let mut name = "";
        let mut counter = 0;

        for tagging in self.taggings() {
            if let Some(header) = tagging.header() {
                counters.push((name, counter));

                name = header;
                counter = 0;
            } else if tagging.chapter().is_some() {
                counter += 1;
            } else {
                unreachable!()
            }
        }

        let mut chapters = self.taggings().filter_map(ChapterGroupTagging::chapter);

        let mut volumes = Vec::new();

        for (name, count) in counters.into_iter().skip(1) {
            let volume_chapters = chapters.by_ref().take(count);

            let volume = Volume {
                name,
                chapters: volume_chapters.collect(),
            };
            volumes.push(volume)
        }

        volumes
    }

    #[tracing::instrument(skip_all)]
    pub async fn chapters(&self, dynasty: &Dynasty) -> crate::Result<Vec<Chapter>> {
        let vec = self.chapters_stream(dynasty).try_collect().await?;
        tracing::trace!("all chapters fetched");
        Ok(vec)
    }

    pub fn chapters_stream<'a>(
        &'a self,
        dynasty: &'a Dynasty,
    ) -> impl futures::TryStream<Ok = Chapter, Error = crate::Error> + 'a {
        futures::stream::iter(self.taggings().filter_map(ChapterGroupTagging::chapter))
            .then(move |meta| Chapter::get(dynasty, &meta.permalink))
            .enumerate()
            .filter_map(|(n, mut chapter)| async move {
                let _ = chapter
                    .iter_mut()
                    .map(|chapter| chapter.set_dynasty_index(n));

                Some(chapter)
            })
    }
}

pub struct Volume<'series> {
    name: &'series str,
    chapters: Vec<&'series ChapterMeta>,
}
