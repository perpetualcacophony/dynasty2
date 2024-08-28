use std::future::IntoFuture;

use futures::future::BoxFuture;

use crate::Dynasty;

use super::{Response, UrlBuilder};

pub trait RequestParams<'a> {
    fn url<'url, 'builder>(
        self,
        builder: &'url mut UrlBuilder<'builder>,
    ) -> &'url mut UrlBuilder<'builder>
    where
        Self: 'builder + Sized,
    {
        builder
    }
}

pub struct Request<'a, R: Response> {
    dynasty: &'a Dynasty,
    params: R::Params<'a>,
}

impl<'a, R: Response> Request<'a, R> {
    pub(crate) fn new(dynasty: &'a Dynasty, params: R::Params<'a>) -> Self {
        Self { dynasty, params }
    }

    pub async fn send(self) -> crate::Result<R> {
        let mut url = UrlBuilder::new(R::PATH);
        self.params.url(&mut url);
        Ok(self.dynasty.http().json(url).await?)
    }
}

impl<'a, R> IntoFuture for Request<'a, R>
where
    R: Response + 'a,
    for<'b> R::Params<'b>: Send,
{
    type Output = crate::Result<R>;
    type IntoFuture = BoxFuture<'a, Self::Output>;

    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.send())
    }
}
