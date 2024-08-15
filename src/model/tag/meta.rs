use crate::Dynasty;

use super::{Tag, Type};

#[derive(serde::Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
pub struct TagMeta {
    pub name: String,

    #[serde(rename = "type")]
    pub type_: Type,

    pub permalink: String,
}

impl TagMeta {
    pub fn slug(&self) -> &str {
        &self.permalink
    }

    pub fn is_series(&self) -> bool {
        self.type_ == Type::Series
    }

    pub fn is_crossover(&self) -> bool {
        self.slug() == "crossover"
    }

    pub fn permalink(&self) -> String {
        format!(
            "{path}/{slug}",
            path = self.type_.path(),
            slug = self.slug()
        )
    }

    pub async fn get(&self, dynasty: &Dynasty) -> crate::Result<Tag> {
        Ok(dynasty.http().json(&self.permalink()).await?)
    }
}
