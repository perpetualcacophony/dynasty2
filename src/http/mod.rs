pub struct Http {
    #[cfg(feature = "reqwest")]
    reqwest: reqwest::Client,
}

impl Http {
    pub const HOST_URL: &str = "https://dynasty-scans.com";

    pub fn host_url() -> url::Url {
        url::Url::parse(Self::HOST_URL).expect("host url should be valid")
    }

    #[cfg(feature = "reqwest")]
    pub async fn json<Json: serde::de::DeserializeOwned>(&self, path: &str) -> Result<Json> {
        Ok(self
            .reqwest
            .get(format!("{host}/{path}.json", host = Self::HOST_URL))
            .send()
            .await?
            .json()
            .await?)
    }
}

pub struct Error {
    #[cfg(feature = "reqwest")]
    inner: reqwest::Error,
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        #[cfg(feature = "reqwest")]
        self.inner.fmt(f)?;

        Ok(())
    }
}

#[cfg(feature = "reqwest")]
impl From<reqwest::Error> for Error {
    fn from(value: reqwest::Error) -> Self {
        Self { inner: value }
    }
}

pub type Result<T, E = Error> = std::result::Result<T, E>;
