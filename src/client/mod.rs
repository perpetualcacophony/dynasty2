use std::fmt::Display;

use serde::de::DeserializeOwned;

use crate::{
    http,
    model::{Doujins, Scanlator, Series},
    response::{request::RequestCore, Request, Response},
    Author, Chapter, Http, Pairing, Slug, Tag,
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
        Ok(Slug::parse(s)?)
    }

    fn request<'a, T: Response + DeserializeOwned + 'a>(&'a self) -> RequestCore<'a, T> {
        RequestCore::new(self, T::PATH)
    }

    pub fn chapter<'a>(&'a self, slug: &'a str) -> impl Request<Resp = Chapter> + 'a {
        self.request().slug(Self::parse_slug(slug).unwrap())
    }

    pub fn series<'a>(&'a self, slug: &'a str) -> impl Request<Resp = Series> + 'a {
        self.request().slug(Self::parse_slug(slug).unwrap())
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
