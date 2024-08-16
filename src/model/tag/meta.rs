use super::Type;
use crate::{Slug, SlugOwned};

#[derive(serde::Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
pub struct TagMeta<Type = super::Type> {
    name: String,

    #[serde(rename = "type")]
    pub type_: Type,

    permalink: SlugOwned,
}

impl<T> TagMeta<T> {
    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn slug(&self) -> Slug {
        self.permalink.as_ref()
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
