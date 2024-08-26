use std::future::IntoFuture;

use futures::future::BoxFuture;

use crate::{Dynasty, Slug};

use super::Series;

pub struct RequestSeries<'a> {
    dynasty: &'a Dynasty,
    slug: Slug<'a>,
}

impl<'a> RequestSeries<'a> {
    pub(crate) fn new(dynasty: &'a Dynasty, slug: Slug<'a>) -> Self {
        Self { dynasty, slug }
    }

    pub async fn send(self) -> crate::Result<Series> {
        todo!()
    }
}

impl<'a> IntoFuture for RequestSeries<'a> {
    type Output = crate::Result<Series>;
    type IntoFuture = BoxFuture<'a, Self::Output>;

    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.send())
    }
}
