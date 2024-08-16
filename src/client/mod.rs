use std::{fmt::Display, str::FromStr};

use crate::{
    http,
    model::{Doujins, Scanlator, Series},
    Author, Chapter, Http, LinkPath, Pairing, Path, Slug, Tag,
};

#[derive(Default, Clone, Debug)]
pub struct Dynasty {
    http: Http,
}

impl Dynasty {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn http(&self) -> &Http {
        &self.http
    }

    pub fn parse_slug(s: &str) -> Result<Slug> {
        Ok(Slug::from_str(s)?)
    }

    pub async fn get_json<Json: serde::de::DeserializeOwned>(
        &self,
        path: Path,
        slug: &Slug,
    ) -> Result<Json> {
        Ok(self.http().json(LinkPath::new(path, slug)).await?)
    }

    pub async fn chapter(&self, slug: &str) -> Result<Chapter> {
        Chapter::get(self, &Self::parse_slug(slug)?).await
    }

    pub async fn tag(&self, slug: &str) -> Result<Tag> {
        Tag::get(self, &Self::parse_slug(slug)?).await
    }

    pub async fn pairing(&self, slug: &str) -> Result<Pairing> {
        Pairing::get(self, &Self::parse_slug(slug)?).await
    }

    pub async fn author(&self, slug: &str) -> Result<Author> {
        Author::get(self, &Self::parse_slug(slug)?).await
    }

    pub async fn scanlator(&self, slug: &str) -> Result<Scanlator> {
        Scanlator::get(self, &Self::parse_slug(slug)?).await
    }

    pub async fn doujins(&self, slug: &str) -> Result<Doujins> {
        Doujins::get(self, &Self::parse_slug(slug)?).await
    }

    pub async fn series(&self, slug: &str) -> Result<Series> {
        Series::get(self, &Self::parse_slug(slug)?).await
    }
}

pub type Result<T, E = Error> = std::result::Result<T, E>;

#[derive(Debug)]
pub enum Error {
    Http(http::Error),
    ParseTag,
    ParseSlug(crate::slug::ParseError),
}

impl From<http::Error> for Error {
    fn from(value: http::Error) -> Self {
        Self::Http(value)
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Http(http) => http.fmt(f),
            Self::ParseTag => f.write_str("bwaa"),
            Self::ParseSlug(err) => err.fmt(f),
        }
    }
}

impl std::error::Error for Error {}

impl From<crate::slug::ParseError> for Error {
    fn from(value: crate::slug::ParseError) -> Self {
        Self::ParseSlug(value)
    }
}
