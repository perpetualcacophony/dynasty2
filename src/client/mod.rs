use std::fmt::Display;

use crate::{
    http,
    model::{Doujins, Scanlator, Series},
    Author, Chapter, Http, Pairing, Path, Slug, Tag,
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

    pub async fn get_json<Json: serde::de::DeserializeOwned>(
        &self,
        path: Path,
        slug: &Slug,
    ) -> Result<Json> {
        Ok(self.http().json(&path.permalink(slug)).await?)
    }

    pub async fn chapter(&self, slug: &Slug) -> Result<Chapter> {
        Chapter::get(self, slug).await
    }

    pub async fn tag(&self, slug: &Slug) -> Result<Tag> {
        Tag::get(self, slug).await
    }

    pub async fn pairing(&self, slug: &Slug) -> Result<Pairing> {
        Pairing::get(self, slug).await
    }

    pub async fn author(&self, slug: &Slug) -> Result<Author> {
        Author::get(self, slug).await
    }

    pub async fn scanlator(&self, slug: &Slug) -> Result<Scanlator> {
        Scanlator::get(self, slug).await
    }

    pub async fn doujins(&self, slug: &Slug) -> Result<Doujins> {
        Doujins::get(self, slug).await
    }

    pub async fn series(&self, slug: &Slug) -> Result<Series> {
        Series::get(self, slug).await
    }
}

pub type Result<T, E = Error> = std::result::Result<T, E>;

#[derive(Debug)]
pub enum Error {
    Http(http::Error),
    ParseTag,
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
        }
    }
}

impl std::error::Error for Error {}
