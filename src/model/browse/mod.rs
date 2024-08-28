mod inner;
pub use inner::BrowseInner as Inner;

mod pages;
pub use pages::Pagination;

mod items;
pub use items::BrowseItems as Items;

#[cfg(feature = "view")]
pub mod view;
#[cfg(feature = "view")]
pub use view::{HasView, View};

mod tag;
pub use tag::{RequestTag, Tag};

#[cfg(feature = "pairings")]
mod pairing;
#[cfg(feature = "pairings")]
pub use pairing::{Pairing, RequestPairing};

mod doujins;
pub use doujins::Doujins;

#[cfg(feature = "scanlators")]
mod scanlator;
#[cfg(feature = "scanlators")]
pub use scanlator::Scanlator;

pub struct BrowseParams<'a, View> {
    slug: crate::Slug<'a>,
    page: crate::Page,
    view: std::marker::PhantomData<View>,
}

impl<'a, V> BrowseParams<'a, V> {
    pub(crate) fn new(slug: crate::Slug<'a>) -> Self {
        Self {
            slug,
            page: Default::default(),
            view: Default::default(),
        }
    }

    pub(crate) fn page(&mut self, page: crate::Page) -> &mut Self {
        self.page = page;
        self
    }
}

impl<'a, V: View> crate::RequestParams<'a> for BrowseParams<'a, V> {
    fn url<'url, 'builder>(
        self,
        builder: &'url mut crate::response::UrlBuilder<'builder>,
    ) -> &'url mut crate::response::UrlBuilder<'builder>
    where
        Self: 'builder + Sized,
    {
        builder.slug(self.slug).page(self.page.into()).view::<V>()
    }
}
