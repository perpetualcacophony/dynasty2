use crate::{Page, Path, Slug};

pub struct PathRequest {
    path: Path,
}

impl PathRequest {
    pub(crate) fn slug(self, slug: Slug<'_>) -> SlugRequest<'_> {
        SlugRequest::new(self, slug)
    }

    pub(crate) fn page(self, page: Page) -> RequestPaginated<Self> {
        RequestPaginated::new(self, page)
    }
}

pub struct SlugRequest<'a> {
    path: PathRequest,
    slug: Slug<'a>,
}

impl<'a> SlugRequest<'a> {
    pub(crate) fn new(path: PathRequest, slug: Slug<'a>) -> Self {
        Self { path, slug }
    }

    pub(crate) fn page(self, page: Page) -> RequestPaginated<Self> {
        RequestPaginated::new(self, page)
    }
}

pub struct RequestPaginated<Req> {
    inner: Req,
    page: Page,
}

impl<Req> RequestPaginated<Req> {
    pub(crate) fn new(inner: Req, page: Page) -> Self {
        Self { inner, page }
    }
}

pub struct RequestViews<Req> {
    inner: Req,
    view: (),
}
