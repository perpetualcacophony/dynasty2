use serde::de::DeserializeOwned;

use crate::{Dynasty, Path, Slug};

use super::UrlBuilder;

mod marker;

pub trait Request {
    type Resp;

    fn dynasty(&self) -> &Dynasty;

    fn send(self) -> impl std::future::Future<Output = crate::Result<Self::Resp>> + Send
    where
        Self: Sized + Copy + Send + Sync + 'static,
        Self::Resp: DeserializeOwned,
    {
        async move { Ok(self.dynasty().http().json(self).await?) }
    }

    fn url<'slug, 'builder>(
        self,
        builder: &'builder mut UrlBuilder<'slug>,
    ) -> &'builder mut UrlBuilder<'slug>
    where
        Self: 'slug;

    fn page(self, page: usize) -> PageRequest<Self>
    where
        Self: marker::NoPage,
    {
        PageRequest { req: self, page }
    }

    fn slug(self, slug: Slug) -> SlugRequest<Self>
    where
        Self: marker::NoSlug,
    {
        SlugRequest { req: self, slug }
    }
}

pub struct RequestCore<'a, T> {
    dynasty: &'a Dynasty,
    url: UrlBuilder<'a>,
    phantom: std::marker::PhantomData<T>,
}

impl<'a, T> RequestCore<'a, T> {
    pub(crate) fn new(dynasty: &'a Dynasty, path: Path) -> Self {
        Self {
            dynasty,
            url: UrlBuilder::new(path),
            phantom: Default::default(),
        }
    }
}

impl<T> marker::NoPage for RequestCore<'_, T> {}
impl<T> marker::NoSlug for RequestCore<'_, T> {}

pub struct SlugRequest<'slug, Req> {
    req: Req,
    slug: Slug<'slug>,
}

impl<'a, Req> Request for SlugRequest<'a, Req>
where
    Req: Request,
{
    type Resp = Req::Resp;

    fn url<'slug, 'builder>(
        self,
        builder: &'builder mut UrlBuilder<'slug>,
    ) -> &'builder mut UrlBuilder<'slug>
    where
        Self: 'slug,
    {
        builder.slug(self.slug)
    }

    fn dynasty(&self) -> &Dynasty {
        self.req.dynasty()
    }
}

impl<T> marker::NoPage for SlugRequest<'_, T> where T: marker::NoPage {}

impl<'a, T> Request for RequestCore<'a, T>
where
    T: DeserializeOwned,
{
    type Resp = T;

    fn url<'slug, 'builder>(
        self,
        builder: &'builder mut UrlBuilder<'slug>,
    ) -> &'builder mut UrlBuilder<'slug>
    where
        Self: 'slug,
    {
        builder
    }

    fn dynasty(&self) -> &Dynasty {
        self.dynasty
    }
}

pub struct PageRequest<Req> {
    req: Req,
    page: usize,
}

impl<Req> Request for PageRequest<Req>
where
    Req: Request,
{
    type Resp = Req::Resp;

    fn url<'slug, 'builder>(
        self,
        builder: &'builder mut UrlBuilder<'slug>,
    ) -> &'builder mut UrlBuilder<'slug>
    where
        Self: 'slug,
    {
        builder.page(self.page)
    }

    fn dynasty(&self) -> &Dynasty {
        self.req.dynasty()
    }
}

impl<T> marker::NoSlug for PageRequest<T> where T: marker::NoSlug {}
