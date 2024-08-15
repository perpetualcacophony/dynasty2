use crate::model::TagType;

pub enum Path {
    Chapter,
    Tag(TagType),
}

impl Path {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Chapter => "chapter",
            Self::Tag(tag) => tag.path(),
        }
    }

    pub fn permalink(&self, slug: &str) -> String {
        format!("{path}/{slug}", path = self.as_str())
    }
}
