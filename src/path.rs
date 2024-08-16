#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Path {
    Chapters,
    Series,
    Authors,
    Anthologies,
    Scanlators,
    Issues,
    Doujins,
    Pairings,
    Tags,
}

impl Path {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Chapters => "chapters",
            Self::Series => "series",
            Self::Authors => "authors",
            Self::Anthologies => "anthologies",
            Self::Scanlators => "scanlators",
            Self::Issues => "issues",
            Self::Doujins => "doujins",
            Self::Pairings => "pairings",
            Self::Tags => "tags",
        }
    }
}

impl std::fmt::Display for Path {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
