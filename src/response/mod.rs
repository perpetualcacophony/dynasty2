use crate::{Path, Slug};

pub mod request;
pub use request::Request;

pub trait Response {
    const PATH: Path;
}

#[derive(Clone, Copy, Debug)]
pub struct UrlBuilder<'a> {
    page: usize,
    path: Option<Path>,
    slug: Option<Slug<'a>>,
}

impl Default for UrlBuilder<'_> {
    fn default() -> Self {
        Self {
            page: 1,
            path: None,
            slug: None,
        }
    }
}

impl<'a> UrlBuilder<'a> {
    pub fn new(path: Path) -> Self {
        Self {
            path: Some(path),
            ..Default::default()
        }
    }

    pub fn slug(&mut self, slug: Slug<'a>) -> &mut Self {
        self.slug = Some(slug);
        self
    }

    pub fn page(&mut self, n: usize) -> &mut Self {
        self.page = n;
        self
    }

    pub fn url(&self) -> url::Url {
        let mut url = crate::Http::host_url();
        url.set_path(&self.path.unwrap());

        if let Some(slug) = self.slug {
            url.set_path(&format!("{old}/{slug}", old = url.path()));
        }

        if self.page != 1 {
            url.set_query(Some(&format!("page={}", self.page)));
        }

        url
    }
}
