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
