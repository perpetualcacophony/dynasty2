use std::future::IntoFuture;

use futures::future::BoxFuture;

use crate::{Dynasty, Slug};

use super::Chapter;

pub struct RequestChapter<'a> {
    dynasty: &'a Dynasty,
    slug: Slug<'a>,
}

impl<'a> RequestChapter<'a> {
    pub(crate) fn new(dynasty: &'a Dynasty, slug: Slug<'a>) -> Self {
        Self { dynasty, slug }
    }

    pub async fn send(self) -> crate::Result<Chapter> {
        todo!()
    }
}

impl<'a> IntoFuture for RequestChapter<'a> {
    type Output = crate::Result<Chapter>;
    type IntoFuture = BoxFuture<'a, Self::Output>;

    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.send())
    }
}
