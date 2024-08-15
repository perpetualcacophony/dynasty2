pub mod series;
pub use series::{Series, Tagging as SeriesTagging};

pub mod anthology;
pub use anthology::Anthology;

mod kind;
pub use kind::GroupingKind as Kind;

use super::{Tag, TagMeta};
use crate::Dynasty;

#[derive(serde::Deserialize, Debug, Clone, PartialEq, Eq, Hash)]
pub struct Grouping {
    #[serde(flatten)]
    pub tag: Tag,

    pub cover: String,

    pub description: Option<String>,
}

impl Grouping {
    pub async fn get(dynasty: &Dynasty, kind: Kind, slug: &str) -> crate::Result<Self> {
        dynasty.get_json(crate::Path::Tag(kind.into()), slug).await
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
}
