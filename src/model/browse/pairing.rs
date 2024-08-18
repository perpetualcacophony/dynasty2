use std::ops::Deref;

use super::{view, Inner};

/// A tag representing a romantic relationship.
#[derive(serde::Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
pub struct Pairing<View = view::Chapters> {
    #[serde(flatten)]
    inner: Inner<View>,
}

impl Pairing {
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

impl<T> crate::Response for Pairing<T> {
    const PATH: crate::Path = crate::Path::new("pairings");
}

impl<View> Deref for Pairing<View> {
    type Target = Inner<View>;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
