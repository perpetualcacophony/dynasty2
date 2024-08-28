use std::ops::Deref;

use crate::{response::request::Request, Dynasty};

use super::{view, BrowseParams, HasView, Inner, View};

#[derive(serde::Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
pub struct Tag<View = view::Chapters> {
    #[serde(flatten)]
    inner: Inner<View>,
}

impl<V: View> crate::Response for Tag<V> {
    const PATH: crate::Path = crate::Path::new("tags");
    type Params<'a> = BrowseParams<'a, V>;
}

impl<T> Deref for Tag<T> {
    type Target = Inner<T>;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl<T> HasView<view::Chapters> for Tag<T> {}
impl<T> HasView<view::Groupings> for Tag<T> {}
impl<T> HasView<view::OneShots> for Tag<T> {}

impl<V: View> Tag<V> {
    pub(crate) fn request<'lily, V2>(&'lily self, dynasty: &'lily Dynasty) -> RequestTag<'lily, V2>
    where
        V2: View,
        Self: HasView<V2>,
    {
        RequestTag::new(dynasty, BrowseParams::new(self.slug()))
    }

    pub fn chapters<'lily>(
        &'lily self,
        dynasty: &'lily Dynasty,
    ) -> RequestTag<'lily, view::Chapters> {
        self.request(dynasty)
    }

    pub fn groupings<'lily>(
        &'lily self,
        dynasty: &'lily Dynasty,
    ) -> RequestTag<'lily, view::Groupings> {
        self.request(dynasty)
    }

    pub fn one_shots<'lily>(
        &'lily self,
        dynasty: &'lily Dynasty,
    ) -> RequestTag<'lily, view::OneShots> {
        self.request(dynasty)
    }
}

pub type RequestTag<'a, View> = Request<'a, Tag<View>>;
