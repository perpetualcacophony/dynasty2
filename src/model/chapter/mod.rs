mod json;
pub use json::ChapterJson as Json;

use crate::{client, Dynasty, Handler};

pub struct Chapter {
    json: Json,
}

impl Chapter {
    pub async fn get(dynasty: &Dynasty, permalink: &str) -> client::Result<Self> {
        Ok(Self {
            json: dynasty
                .http()
                .json(&format!("chapters/{permalink}"))
                .await?,
        })
    }

    pub fn pages(&self) -> impl Iterator<Item = Page> {
        self.json.pages.iter().map(|page| Page {
            filename: &page.name,
            url: &page.url,
        })
    }
}

impl Handler for Chapter {
    const PATH: &str = "chapters";
    type Json = Json;

    fn from_json(json: Self::Json) -> Self {
        Self { json }
    }
}

pub struct Page<'ch> {
    pub filename: &'ch str,
    url: &'ch str,
}

impl<'ch> Page<'ch> {
    pub fn url(&self) -> String {
        format!(
            "{host}{path}",
            host = crate::Http::HOST_URL,
            path = self.url
        )
    }
}
