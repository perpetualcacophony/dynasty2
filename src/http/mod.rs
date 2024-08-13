#[derive(Default, Clone, Debug)]
pub struct Http {
    #[cfg(feature = "reqwest")]
    reqwest: reqwest::Client,
}

impl Http {
    pub const HOST_URL: &str = "https://dynasty-scans.com";

    pub fn new() -> Self {
        Self::default()
    }

    pub fn host_url() -> url::Url {
        url::Url::parse(Self::HOST_URL).expect("host url should be valid")
    }

    pub fn construct_url(path: &str) -> Result<url::Url, url::ParseError> {
        Self::host_url().join(path)
    }

    #[cfg(feature = "reqwest")]
    #[tracing::instrument(skip(self))]
    pub async fn json<Json: serde::de::DeserializeOwned>(&self, path: &str) -> Result<Json> {
        let mut url = Self::construct_url(path).unwrap();
        url.set_path(&format!("{path}.json"));
        tracing::debug!(%url);

        let json: Result<Json> =
            backoff::future::retry(backoff::ExponentialBackoff::default(), || async {
                Ok(self
                    .reqwest
                    .get(url.as_str())
                    .send()
                    .await
                    .map_err(Error::from)?
                    .json::<Json>()
                    .await
                    .map_err(|err| JsonError {
                        path: path.to_owned(),
                        client: err.into(),
                    })
                    .map_err(Error::from)?)
            })
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

#[derive(Debug)]
pub struct JsonError {
    path: String,
    client: ClientError,
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
