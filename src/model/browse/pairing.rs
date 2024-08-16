use std::ops::Deref;

use crate::Dynasty;

use super::Inner;

#[derive(serde::Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
pub struct Pairing {
    #[serde(flatten)]
    inner: Inner,
}

impl Pairing {
    pub async fn get(dynasty: &Dynasty, slug: &str) -> crate::Result<Self> {
        dynasty
            .get_json(crate::Path::Tag(crate::model::TagType::Pairing), slug)
            .await
    }

    pub fn partners(&self) -> impl Iterator<Item = &str> {
        self.name().split(" x ")
    }

    pub fn couple(&self) -> Option<[&str; 2]> {
        if self.partners().count() > 2 {
            return None;
        }

        let mut partners = self.partners();

        Some([partners.next()?, partners.next()?])
    }
}

impl Deref for Pairing {
    type Target = Inner;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
