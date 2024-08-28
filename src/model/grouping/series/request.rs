use crate::{response::request::Request, RequestParams, Slug};

pub struct SeriesParams<'a> {
    slug: Slug<'a>,
}

impl<'a> SeriesParams<'a> {
    pub(crate) fn new(slug: Slug<'a>) -> Self {
        Self { slug }
    }
}

impl<'a> RequestParams<'a> for SeriesParams<'a> {
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

pub type RequestSeries<'a> = Request<'a, super::Series>;
