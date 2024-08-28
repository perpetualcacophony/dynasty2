use std::fmt::Display;

use crate::{
    http,
    model::{
        author::{AuthorParams, RequestAuthor},
        chapter::ChapterParams,
        grouping::{series::SeriesParams, RequestSeries},
        RequestChapter,
    },
    response::{request::Request, Response},
    Http, Slug, ToSlug,
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

    pub fn request<'a, R: Response>(&'a self, params: R::Params<'a>) -> Request<'a, R> {
        Request::new(self, params)
    }

    pub fn chapter<'a>(&'a self, slug: &'a impl ToSlug) -> Result<RequestChapter<'a>> {
        Ok(self.request(ChapterParams::new(slug.to_slug()?)))
    }

    pub fn series<'a>(&'a self, slug: &'a impl ToSlug) -> Result<RequestSeries<'a>> {
        Ok(self.request(SeriesParams::new(slug.to_slug()?)))
    }

    pub fn author<'a>(&'a self, slug: &'a impl ToSlug) -> Result<RequestAuthor<'a>> {
        Ok(self.request(AuthorParams::new(slug.to_slug()?)))
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
