use crate::Dynasty;

use super::{Tag, Type};

#[derive(serde::Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
pub struct TagMeta<Type = super::Type> {
    name: String,

    #[serde(rename = "type")]
    pub type_: Type,

    permalink: String,
}

impl<T> TagMeta<T> {
    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn slug(&self) -> &str {
        &self.permalink
    }

    pub fn is_series(&self) -> bool
    where
        T: PartialEq<Type>,
    {
        self.type_ == Type::Series
    }

    pub fn is_crossover(&self) -> bool {
        self.slug() == "crossover"
    }
}

impl TagMeta {
    pub fn permalink(&self) -> String {
        format!(
            "{path}/{slug}",
            path = self.type_.path(),
            slug = self.slug()
        )
    }

    pub async fn get(&self, dynasty: &Dynasty) -> crate::Result<Tag> {
        dynasty.tag(self.type_, self.slug()).await
    }
}
