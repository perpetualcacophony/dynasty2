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
