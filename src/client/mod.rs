use crate::{http, model::Series, Handler, Http};

pub struct Dynasty {
    http: Http,
}

impl Dynasty {
    pub fn http(&self) -> &Http {
        &self.http
    }

    pub async fn series(&self, permalink: &str) -> Result<Series> {
        Series::get(self, permalink).await
    }
}

pub type Result<T, E = Error> = std::result::Result<T, E>;

pub enum Error {
    Http(http::Error),
}

impl From<http::Error> for Error {
    fn from(value: http::Error) -> Self {
        Self::Http(value)
    }
}
