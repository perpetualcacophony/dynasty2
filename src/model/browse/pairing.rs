use std::ops::Deref;

use crate::Dynasty;

use super::Inner;

#[derive(serde::Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
pub struct Pairing {
    inner: Inner,
}

impl Pairing {
    pub async fn get(dynasty: &Dynasty, slug: &str) -> crate::Result<Self> {
        dynasty
            .get_json(crate::Path::Tag(crate::model::TagType::Pairing), slug)
            .await
    }

    pub fn partners(&self) -> (&str, &str) {
        self.name()
            .split_once(" x ")
            .expect("pairing tag should be named {partner} x {partner}")
    }
}

impl Deref for Pairing {
    type Target = Inner;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
