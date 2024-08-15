#[derive(serde::Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
pub struct BrowsePages {
    current_page: usize,
    total_pages: usize,
}

impl BrowsePages {
    pub fn current(&self) -> usize {
        self.current_page
    }

    pub fn total(&self) -> usize {
        self.total_pages
    }
}
