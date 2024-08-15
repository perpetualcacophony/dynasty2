use crate::model::ChapterMeta;

pub struct Volume<'series> {
    pub name: &'series str,
    pub chapters: Vec<&'series ChapterMeta>,
}
