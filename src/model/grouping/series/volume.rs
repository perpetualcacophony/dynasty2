use crate::model::ChapterMeta;

/// A collection of chapters within a series.
///
/// This object does not have a counterpart in the Dynasty API,
/// and is provided for convenience.
pub struct Volume<'series> {
    pub name: &'series str,
    pub chapters: Vec<&'series ChapterMeta>,
}
