use super::Index;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ChapterId<'a> {
    Text(&'a str),
    Index(Index),
}

impl PartialOrd for ChapterId<'_> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        if let Self::Index(self_index) = self {
            if let Self::Index(other_index) = other {
                return Some(self_index.cmp(other_index));
            }
        }

        None
    }
}

impl<'a> ChapterId<'a> {
    pub fn parse(s: &'a str) -> Self {
        Self::from(s)
    }

    pub fn to_dynasty(self) -> String {
        match self {
            Self::Index(index) => index.to_dynasty(),
            Self::Text(text) => text.to_owned(),
        }
    }

    pub fn permalink(&self, series: &str) -> String {
        format!("{series}_{chapter_id}", chapter_id = self.to_dynasty())
    }

    pub fn from_permalink(permalink: &'a str, series: &str) -> Self {
        let raw = permalink
            .strip_prefix(&format!("{series}_"))
            .expect("permalink should start with series");

        if let Some(index) = raw.strip_prefix("ch") {
            Self::Index(index.parse().expect("should be valid index"))
        } else {
            Self::Text(raw)
        }
    }

    pub fn index(&self) -> Option<Index> {
        if let Self::Index(index) = self {
            Some(*index)
        } else {
            None
        }
    }
}

impl<'a> From<&'a str> for ChapterId<'a> {
    fn from(value: &'a str) -> Self {
        if let Ok(index) = value.parse() {
            Self::Index(index)
        } else {
            Self::Text(value)
        }
    }
}
