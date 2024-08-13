use crate::Dynasty;

pub trait Handler {
    const PATH: &str;
    type Json;

    fn path(permalink: &str) -> String {
        Self::PATH.to_owned() + "/" + permalink
    }

    fn from_json(json: Self::Json) -> Self;

    async fn get(dynasty: &Dynasty, permalink: &str) -> crate::client::Result<Self>
    where
        Self::Json: serde::de::DeserializeOwned,
        Self: Sized,
    {
        Ok(Self::from_json(
            dynasty
                .http()
                .json::<Self::Json>(&Self::path(permalink))
                .await?,
        ))
    }
}
