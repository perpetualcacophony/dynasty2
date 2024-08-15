use futures::{StreamExt, TryStreamExt};

use crate::Dynasty;

use super::{Chapter, ParseTag, Tag};

pub struct Series {
    tag: Tag,
}

impl Series {
    pub fn title(&self) -> &str {
        &self.tag.name
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
        futures::stream::iter(self.tag.chapters())
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

impl ParseTag for Series {
    const TYPE: super::TagType = super::TagType::Series;

    fn from_tag(tag: Tag) -> Self
    where
        Self: Sized,
    {
        Self { tag }
    }
}
