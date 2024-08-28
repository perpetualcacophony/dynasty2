use crate::{response::request::Request, RequestParams, Slug};

pub struct ChapterParams<'a> {
    slug: Slug<'a>,
}

impl<'a> ChapterParams<'a> {
    pub(crate) fn new(slug: Slug<'a>) -> Self {
        Self { slug }
    }
}

impl<'a> RequestParams<'a> for ChapterParams<'a> {
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

pub type RequestChapter<'a> = Request<'a, super::Chapter>;
