use crate::{http, model::Series, Chapter, Handler, Http, Path};

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

    pub async fn series(&self, slug: &str) -> Result<Series> {
        Series::get(self, slug).await
    }

    pub async fn get_json<Json: serde::de::DeserializeOwned>(
        &self,
        path: Path,
        slug: &str,
    ) -> Result<Json> {
        Ok(self.http().json(&path.permalink(slug)).await?)
    }

    pub async fn chapter(&self, slug: &str) -> Result<Chapter> {
        Chapter::get(self, slug).await
    }
}

pub type Result<T, E = Error> = std::result::Result<T, E>;

#[derive(Debug)]
pub enum Error {
    Http(http::Error),
}

impl From<http::Error> for Error {
    fn from(value: http::Error) -> Self {
        Self::Http(value)
    }
}
