mod json;
pub use json::ChapterJson as Json;

pub mod index;
pub use index::ChapterIndex as Index;

pub mod id;
pub use id::ChapterId as Id;

use crate::Handler;

pub struct Chapter {
    json: Json,
    pub dynasty_index: Option<usize>,
}

impl Chapter {
    pub fn pages(&self) -> impl Iterator<Item = Page> {
        self.json.pages.iter().map(|page| Page {
            filename: &page.name,
            permalink: &page.url,
        })
    }

    pub fn title(&self) -> &str {
        &self.json.title
    }

    pub fn permalink(&self) -> &str {
        &self.json.permalink
    }

    pub fn set_dynasty_index(&mut self, index: usize) {
        self.dynasty_index = Some(index)
    }

    pub fn id(&self) -> Id {
        Id::from_permalink(self.permalink(), todo!())
    }

    pub fn index(&self) -> Option<Index> {
        self.id().index()
    }
}

impl Handler for Chapter {
    const PATH: &str = "chapters";
    type Json = Json;

    fn from_json(json: Self::Json) -> Self {
        Self {
            json,
            dynasty_index: None,
        }
    }
}

pub struct Page<'ch> {
    pub filename: &'ch str,
    permalink: &'ch str,
}

impl<'ch> Page<'ch> {
    pub fn url(&self) -> String {
        format!(
            "{host}{path}",
            host = crate::Http::HOST_URL,
            path = self.permalink
        )
    }
}
