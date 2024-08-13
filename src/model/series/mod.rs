mod json;
use futures::{StreamExt, TryStreamExt};
pub use json::SeriesJson as Json;
use json::TaggingsItem;

use crate::{Dynasty, Handler};

use super::Chapter;

pub struct Series {
    json: Json,
}

impl Series {
    pub fn title(&self) -> &str {
        &self.json.name
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
        futures::stream::iter(self.json.taggings.iter()).filter_map(move |tagging| async move {
            match tagging {
                TaggingsItem::Chapter {
                    title: _,
                    ref permalink,
                    released_on: _,
                } => Some(Chapter::get(dynasty, permalink).await),
                _ => None,
            }
        })
    }
}

impl Handler for Series {
    const PATH: &str = "series";
    type Json = Json;

    fn from_json(json: Self::Json) -> Self {
        Self { json }
    }
}
