use crate::ImageLink;

#[derive(serde::Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
pub struct Page {
    name: String,
    url: ImageLink,
}

impl Page {
    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn image_url(&self) -> url::Url {
        self.url.url()
    }
}
