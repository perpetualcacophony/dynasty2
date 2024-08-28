use std::ops::Deref;

use crate::{response::request::Request, Dynasty, RequestParams, Slug};

use super::{view, HasView, Inner, View};

#[derive(serde::Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
pub struct Tag<View = view::Chapters> {
    #[serde(flatten)]
    inner: Inner<View>,
}

impl<V: View> crate::Response for Tag<V> {
    const PATH: crate::Path = crate::Path::new("tags");
    type Params<'a> = TagParams<'a, V>;
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
        RequestTag::new(dynasty, TagParams::new(self.slug()))
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

pub struct TagParams<'a, View> {
    slug: Slug<'a>,
    page: usize,
    view: std::marker::PhantomData<View>,
}

impl<'a, T> TagParams<'a, T> {
    pub(crate) fn new(slug: Slug<'a>) -> Self {
        Self {
            slug,
            page: 1,
            view: Default::default(),
        }
    }
}

impl<'a, V: View> RequestParams<'a> for TagParams<'a, V> {
    fn url<'url, 'builder>(
        self,
        builder: &'url mut crate::response::UrlBuilder<'builder>,
    ) -> &'url mut crate::response::UrlBuilder<'builder>
    where
        Self: 'builder + Sized,
    {
        builder.slug(self.slug).page(self.page).view::<V>()
    }
}

pub type RequestTag<'a, View> = Request<'a, Tag<View>>;
