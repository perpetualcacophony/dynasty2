use std::fmt::Display;

use serde::de::DeserializeOwned;

use crate::{
    http,
    model::{author::RequestAuthor, grouping::RequestSeries, Doujins, RequestChapter, Series},
    response::{request::RequestCore, Request, Response},
    Author, Chapter, Http, Slug, Tag, ToSlug,
};

#[cfg(feature = "pairings")]
use crate::{model::browse::RequestPairing, Pairing};

#[cfg(feature = "scanlators")]
use crate::model::browse::Scanlator;

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
        Ok(Slug::parse(s)?)
    }

    fn request<'a, T: Response + DeserializeOwned + 'a>(&'a self) -> Request<'a, T> {
        Request::new(self, T::PATH)
    }

    pub fn chapter<'a>(&'a self, slug: &'a impl ToSlug) -> Result<RequestChapter<'a>> {
        Ok(RequestChapter::new(self, slug.to_slug()?))
    }

    pub fn series<'a>(&'a self, slug: &'a impl ToSlug) -> Result<RequestSeries<'a>> {
        Ok(RequestSeries::new(self, slug.to_slug()?))
    }

    pub fn author<'a>(&'a self, slug: &'a impl ToSlug) -> Result<RequestAuthor<'a>> {
        Ok(RequestAuthor::new(self, slug.to_slug()?))
    }

    #[cfg(feature = "pairings")]
    pub fn pairing<'a>(&'a self, slug: &'a impl ToSlug) -> Result<RequestPairing<'a>> {
        Ok(RequestPairing::new(self, slug.to_slug()?))
    }
}

pub type Result<T, E = Error> = std::result::Result<T, E>;

#[derive(Debug)]
pub enum Error {
    Http(http::Error),
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
            Self::Http(err) => err.fmt(f),
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
