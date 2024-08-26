use std::future::IntoFuture;

use futures::future::BoxFuture;

use crate::{Dynasty, Slug};

use super::Author;

pub struct RequestAuthor<'a> {
    dynasty: &'a Dynasty,
    slug: Slug<'a>,
}

impl<'a> RequestAuthor<'a> {
    pub(crate) fn new(dynasty: &'a Dynasty, slug: Slug<'a>) -> Self {
        Self { dynasty, slug }
    }

    pub async fn send(self) -> crate::Result<Author> {
        todo!()
    }
}

impl<'a> IntoFuture for RequestAuthor<'a> {
    type Output = crate::Result<Author>;
    type IntoFuture = BoxFuture<'a, Self::Output>;

    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.send())
    }
}
