use super::{ParseError, Slug};

pub trait ToSlug: Sized {
    fn to_slug(&self) -> Result<Slug, ParseError>;
}

impl ToSlug for Slug<'_> {
    fn to_slug(&self) -> Result<Slug, ParseError> {
        Ok(*self)
    }
}

impl ToSlug for &str {
    fn to_slug(&self) -> Result<Slug, ParseError> {
        Slug::parse(self)
    }
}
