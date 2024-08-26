use std::future::IntoFuture;

use futures::future::LocalBoxFuture;
use serde::de::DeserializeOwned;

use crate::{Dynasty, Path, Slug};

use super::UrlBuilder;

mod marker;

mod request2;

pub trait RequestCore {
    type Resp;

    fn url<'a, 'builder>(
        self,
        builder: &'a mut UrlBuilder<'builder>,
    ) -> &'a mut UrlBuilder<'builder>
    where
        Self: 'builder + Sized,
    {
        builder
    }
}

pub struct Request<'a, Resp, Core = RequestBase<'a, Resp>> {
    dynasty: &'a Dynasty,
    core: Core,
    resp: std::marker::PhantomData<Resp>,
}

impl<'a, Resp> Request<'a, Resp> {
    pub(crate) fn new(dynasty: &'a Dynasty, path: Path) -> Self {
        Self {
            dynasty,
            core: RequestBase::new(dynasty, path),
            resp: Default::default(),
        }
    }
}

impl<'a, Resp, Core> Request<'a, Resp, Core>
where
    Core: RequestCore<Resp = Resp>,
{
    pub fn page(self, page: usize) -> Request<'a, Resp, PageRequest<Core>>
    where
        Core: marker::NoPage,
    {
        Request {
            dynasty: self.dynasty,
            core: PageRequest {
                req: self.core,
                page,
            },
            resp: Default::default(),
        }
    }

    pub fn slug(self, slug: Slug) -> Request<'a, Resp, SlugRequest<Core>>
    where
        Core: marker::NoSlug,
    {
        Request {
            dynasty: self.dynasty,
            core: SlugRequest {
                req: self.core,
                slug,
            },
            resp: Default::default(),
        }
    }

    pub fn url<'b, 'url>(self, url: &'url mut UrlBuilder<'b>) -> &'url mut UrlBuilder<'b>
    where
        Core: 'b,
    {
        self.core.url(url)
    }

    pub async fn send(self) -> crate::Result<Resp>
    where
        Core: Send + Sync + 'static,
        Resp: DeserializeOwned,
    {
        Ok(self.dynasty.http().json(self).await?)
    }
}

impl<'a, Resp, Core> IntoFuture for Request<'a, Resp, Core>
where
    Core: RequestCore<Resp = Resp> + Send + Sync + 'static,
    Resp: DeserializeOwned + 'a,
{
    type Output = crate::Result<Resp>;
    type IntoFuture = LocalBoxFuture<'a, Self::Output>;

    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.send())
    }
}

#[derive(Copy, Clone)]
pub struct RequestBase<'a, T> {
    dynasty: &'a Dynasty,
    url: UrlBuilder<'a>,
    phantom: std::marker::PhantomData<T>,
}

impl<'a, T> RequestBase<'a, T> {
    pub(crate) fn new(dynasty: &'a Dynasty, path: Path) -> Self {
        Self {
            dynasty,
            url: UrlBuilder::new(path),
            phantom: Default::default(),
        }
    }
}

impl<T> marker::NoPage for RequestBase<'_, T> {}
impl<T> marker::NoSlug for RequestBase<'_, T> {}

impl<'a, T> RequestCore for RequestBase<'a, T>
where
    T: DeserializeOwned,
{
    type Resp = T;
}

#[derive(Copy, Clone)]
pub struct SlugRequest<'slug, Req> {
    req: Req,
    slug: Slug<'slug>,
}

impl<'a, Req> RequestCore for SlugRequest<'a, Req>
where
    Req: RequestCore,
{
    type Resp = Req::Resp;

    fn url<'b, 'builder>(
        self,
        builder: &'b mut UrlBuilder<'builder>,
    ) -> &'b mut UrlBuilder<'builder>
    where
        Self: 'builder + Sized,
    {
        builder.slug(self.slug)
    }
}

impl<T> marker::NoPage for SlugRequest<'_, T> where T: marker::NoPage {}

#[derive(Copy, Clone)]
pub struct PageRequest<Req> {
    req: Req,
    page: usize,
}

impl<Req> RequestCore for PageRequest<Req>
where
    Req: RequestCore,
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
}

impl<T> marker::NoSlug for PageRequest<T> where T: marker::NoSlug {}
