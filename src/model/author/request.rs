use crate::{response::request::Request, RequestParams, Slug};

use super::Author;

pub struct AuthorParams<'a> {
    slug: Slug<'a>,
}

impl<'a> AuthorParams<'a> {
    pub(crate) fn new(slug: Slug<'a>) -> Self {
        Self { slug }
    }
}

pub type RequestAuthor<'a> = Request<'a, Author>;

impl<'a> RequestParams<'a> for AuthorParams<'a> {
    fn url<'url, 'builder>(
        self,
        builder: &'url mut crate::response::UrlBuilder<'builder>,
    ) -> &'url mut crate::response::UrlBuilder<'builder>
    where
        Self: 'builder + Sized,
    {
        builder.slug(self.slug)
    }
}
