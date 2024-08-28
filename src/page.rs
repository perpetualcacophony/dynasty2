#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Page(usize);

impl Page {
    pub(crate) fn from_usize(n: usize) -> Self {
        Self(n)
    }
}

impl Default for Page {
    fn default() -> Self {
        Self(1)
    }
}

impl From<Page> for usize {
    fn from(value: Page) -> Self {
        value.0
    }
}
