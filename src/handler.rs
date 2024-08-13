use crate::Dynasty;

pub trait Handler {
    const PATH: &str;
    type Json;

    fn permalink(slug: &str) -> String {
        Self::PATH.to_owned() + "/" + slug
    }

    fn from_json(json: Self::Json) -> Self;

    #[tracing::instrument(skip(dynasty))]
    async fn get(dynasty: &Dynasty, slug: &str) -> crate::client::Result<Self>
    where
        Self::Json: serde::de::DeserializeOwned,
        Self: Sized,
    {
        Ok(Self::from_json(
            dynasty
                .http()
                .json::<Self::Json>(&Self::permalink(slug))
                .await?,
        ))
    }
}
