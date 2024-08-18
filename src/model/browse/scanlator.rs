use std::ops::Deref;

use super::{view, Inner};

/// Represents a single scanlator or scanlation group and their translated chapters.
#[derive(serde::Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
pub struct Scanlator<View = view::Chapters> {
    #[serde(flatten)]
    inner: Inner<View>,
}

impl<View> Deref for Scanlator<View> {
    type Target = Inner<View>;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl crate::Response for Scanlator {
    const PATH: crate::Path = crate::Path::new("scanlators");
}
