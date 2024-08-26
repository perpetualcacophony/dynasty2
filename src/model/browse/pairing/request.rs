use crate::{Dynasty, Page, Slug};

use super::Pairing;

pub struct RequestPairing<'a> {
    dynasty: &'a Dynasty,
    slug: Slug<'a>,
    page: Page,
    view: PairingView,
}

impl<'a> RequestPairing<'a> {
    pub(crate) fn new(dynasty: &'a Dynasty, slug: Slug<'a>) -> Self {
        Self {
            dynasty,
            slug,
            page: Page::default(),
            view: PairingView::default(),
        }
    }

    pub fn page(mut self, page: Page) -> Self {
        self.page = page;
        self
    }

    pub fn view(mut self, view: PairingView) -> Self {
        self.view = view;
        self
    }

    pub async fn send(self) -> crate::Result<Pairing> {}
}

#[derive(Default)]
pub enum PairingView {
    #[default]
    Chapters,

    Groupings,

    OneShots,
}
