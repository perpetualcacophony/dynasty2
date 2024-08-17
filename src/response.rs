use serde::de::DeserializeOwned;

use crate::{Dynasty, LinkPath, Path, Result, Slug};

pub trait Response {
    const PATH: Path;

    async fn get(dynasty: &Dynasty, slug: Slug<'_>) -> Result<Self>
    where
        Self: Sized + DeserializeOwned,
    {
        Ok(dynasty.http().json(LinkPath::new(Self::PATH, slug)).await?)
    }
}
