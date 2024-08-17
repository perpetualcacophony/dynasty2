#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Path {
    inner: &'static str,
}

impl Path {
    pub const fn as_str(&self) -> &str {
        self.inner
    }

    pub const fn new(s: &'static str) -> Self {
        assert!(s.is_ascii());
        Self { inner: s }
    }
}

impl std::fmt::Display for Path {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
