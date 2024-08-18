use crate::response::Request;
use std::fmt::Display;

#[derive(Default, Clone, Debug)]
pub struct Http {
    #[cfg(feature = "reqwest")]
    reqwest: reqwest::Client,
}

use crate::response::UrlBuilder;

impl Http {
    pub const HOST_URL: &str = "https://dynasty-scans.com";

    pub fn new() -> Self {
        Self::default()
    }

    pub fn host_url() -> url::Url {
        url::Url::parse(Self::HOST_URL).expect("host url should be valid")
    }

    pub fn request_url<T>(request: impl Request<Resp = T>) -> url::Url {
        let mut url = UrlBuilder::default();
        request.url(&mut url);
        let mut url = url.url();
        url.set_path(&format!("{}.json", url.path()));
        url
    }

    #[cfg(feature = "reqwest")]
    #[tracing::instrument(skip(self, request))]
    pub async fn json<Json: serde::de::DeserializeOwned>(
        &self,
        request: impl Request<Resp = Json>,
    ) -> Result<Json> {
        let url = Self::request_url(request);
        tracing::debug!(%url);

        let path = url.path().to_owned();

        let json: Result<Json> = backoff::future::retry_notify(
            backoff::ExponentialBackoff::default(),
            || async {
                Ok(self
                    .reqwest
                    .get(url.as_str())
                    .send()
                    .await
                    .map_err(Error::from)?
                    .json::<Json>()
                    .await
                    .map_err(|err| JsonError {
                        path: path.clone(),
                        client: err.into(),
                    })
                    .map_err(Error::from)?)
            },
            |err, time| tracing::warn!(?err),
        )
        .await;

        let json = json?;

        tracing::trace!("done");

        Ok(json)
    }
}

#[derive(Debug)]
pub enum Error {
    Client(ClientError),
    Json(JsonError),
}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Client(client) => client.fmt(f),
            Self::Json(json) => json.fmt(f),
        }
    }
}

#[derive(Debug)]
pub struct JsonError {
    path: String,
    client: ClientError,
}

impl std::fmt::Display for JsonError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.client.fmt(f)
    }
}

#[derive(Debug)]
pub struct ClientError {
    #[cfg(feature = "reqwest")]
    inner: reqwest::Error,
}

impl std::fmt::Display for ClientError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        #[cfg(feature = "reqwest")]
        self.inner.fmt(f)?;

        Ok(())
    }
}

impl From<ClientError> for Error {
    fn from(value: ClientError) -> Self {
        Self::Client(value)
    }
}

impl From<JsonError> for Error {
    fn from(value: JsonError) -> Self {
        Self::Json(value)
    }
}

#[cfg(feature = "reqwest")]
impl From<reqwest::Error> for Error {
    fn from(value: reqwest::Error) -> Self {
        ClientError::from(value).into()
    }
}

#[cfg(feature = "reqwest")]
impl From<reqwest::Error> for ClientError {
    fn from(value: reqwest::Error) -> Self {
        ClientError { inner: value }
    }
}

pub type Result<T, E = Error> = std::result::Result<T, E>;
