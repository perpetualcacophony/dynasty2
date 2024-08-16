use std::fmt::Display;

pub mod errors;
pub use errors::ParseSlugError as ParseError;

mod owned;
pub use owned::SlugOwned;

/// Uniquely identifies a response from a given category.
#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
pub struct Slug<'a> {
    inner: &'a str,
}

impl<'a> Slug<'a> {
    pub fn as_str(&self) -> &'a str {
        self.inner
    }

    pub fn parse(s: &'a str) -> Result<Self, ParseError> {
        Self::try_from(s)
    }

    fn to_owned(self) -> SlugOwned {
        SlugOwned::from_ref(self)
    }
}

impl<'a> TryFrom<&'a str> for Slug<'a> {
    type Error = ParseError;

    fn try_from(value: &'a str) -> Result<Self, Self::Error> {
        if value.is_empty() {
            return Err(ParseError::new(value).empty_input());
        }

        if value.starts_with('_') || value.ends_with('_') {
            return Err(ParseError::new(value).hanging_underscore());
        }

        if value.chars().all(|ch| ch == '_') {
            return Err(ParseError::new(value).only_underscores());
        }

        #[allow(clippy::match_like_matches_macro)]
        let character_illegal = |ch: &char| match ch {
            'A'..='Z' | ' '..='/' | ':'..='@' | '['..='^' | '`' | '{'..='}' => true,
            _ => false,
        };

        if let Some(illegal) = value.chars().find(character_illegal) {
            return Err(ParseError::new(value).illegal_character(illegal));
        }

        Ok(Self { inner: value })
    }
}

impl Display for Slug<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}

impl<'a> PartialEq<&'a str> for Slug<'a> {
    fn eq(&self, other: &&'a str) -> bool {
        self.as_str().eq(*other)
    }
}

#[cfg(test)]
mod tests {
    use super::Slug;

    mod parse {
        use super::Slug;

        #[test]
        fn valid() {
            Slug::parse("arai_sumiko").unwrap();
        }

        #[test]
        #[should_panic]
        fn all_underscores() {
            Slug::parse("_______").unwrap();
        }

        #[test]
        #[should_panic]
        fn empty_input() {
            Slug::parse(" ").unwrap();
        }

        #[test]
        #[should_panic]
        fn illegal_hyphen() {
            Slug::parse("arai-sumiko").unwrap();
        }

        #[test]
        #[should_panic]
        fn illegal_caps() {
            Slug::parse("arai_Sumiko").unwrap();
        }
    }
}
