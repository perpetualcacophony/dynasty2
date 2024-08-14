pub mod json;
pub use json::TagJson as Json;

use crate::Dynasty;

use super::Series;

#[derive(Clone, Debug, PartialEq, Eq, serde::Deserialize)]
#[serde(transparent)]
pub struct Tag {
    json: Json,
}

impl Tag {
    pub fn from_json(json: Json) -> Self {
        Self { json }
    }

    pub fn name(&self) -> &str {
        &self.json.name
    }

    pub fn slug(&self) -> &str {
        &self.json.permalink
    }

    pub fn is_crossover(&self) -> bool {
        self.slug() == "crossover"
    }

    pub fn permalink(&self) -> String {
        format!(
            "{path}/{slug}",
            path = self.json.type_.path(),
            slug = self.slug()
        )
    }

    pub fn is_series(&self) -> bool {
        self.json.type_ == TagType::Series
    }

    pub async fn series(&self, dynasty: &Dynasty) -> Option<crate::Result<Series>> {
        if self.is_series() {
            Some(dynasty.series(self.slug()).await)
        } else {
            None
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Copy)]
pub enum TagType {
    Author,
    General,
    Series,
    Scanlator,
    Pairing,
    Doujin,
}

impl TagType {
    pub fn parse(s: &str) -> Result<Self, &str> {
        match s {
            "Author" => Ok(Self::Author),
            "General" => Ok(Self::General),
            "Series" => Ok(Self::Series),
            "Scanlator" => Ok(Self::Scanlator),
            "Pairing" => Ok(Self::Pairing),
            "Doujin" => Ok(Self::Doujin),
            _ => Err(s),
        }
    }

    pub fn path(&self) -> &'static str {
        match self {
            Self::Author => "authors",
            Self::Doujin => "doujins",
            Self::Series => "series",
            Self::Scanlator => "scanlators",
            Self::Pairing => "pairings",
            Self::General => "tags",
        }
    }
}

impl<'a> TryFrom<&'a str> for TagType {
    type Error = &'a str;

    fn try_from(value: &'a str) -> Result<Self, Self::Error> {
        Self::parse(value)
    }
}

impl<'de> serde::Deserialize<'de> for TagType {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        Self::parse(&s).map_err(serde::de::Error::custom)
    }
}
