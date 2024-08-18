use std::ops::Deref;

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

impl Deref for Path {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        self.as_str()
    }
}

impl std::fmt::Display for Path {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
