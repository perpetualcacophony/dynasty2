use std::ops::Deref;

use super::Inner;

use crate::model::ChapterMeta;

mod tagging;
pub use request::SeriesParams;
pub use tagging::SeriesTagging as Tagging;

mod request;
pub use request::RequestSeries;

mod volume;
pub use volume::Volume;

/// A collection of chapters organized in volumes, following a narrative.
#[derive(serde::Deserialize, Debug, Clone, PartialEq, Eq, Hash)]
pub struct Series {
    #[serde(flatten)]
    grouping: Inner,

    taggings: Vec<Tagging>,
}

impl Series {
    /// Returns volume headers and chapter metadata in the layout
    /// directly received from Dynasty.
    ///
    /// This method is highly cumbersome, and most users will be better
    /// served by [`Series::volumes`] for organizing chapters into volumes
    /// or [`Series::chapters`] for simply iterating through all chapters.
    pub fn taggings(&self) -> impl Iterator<Item = &Tagging> {
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

        let mut chapters = self.taggings().filter_map(Tagging::chapter);

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
    pub fn chapters(&self) -> impl Iterator<Item = &ChapterMeta> + '_ {
        self.taggings().filter_map(Tagging::chapter)
    }
}

impl crate::Response for Series {
    const PATH: crate::Path = crate::Path::new("series");
    type Params<'a> = SeriesParams<'a>;
}

impl Deref for Series {
    type Target = Inner;

    fn deref(&self) -> &Self::Target {
        &self.grouping
    }
}
