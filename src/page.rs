#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Page(usize);

impl Default for Page {
    fn default() -> Self {
        Self(1)
    }
}
