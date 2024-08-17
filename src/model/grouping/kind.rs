use crate::model::TagType;

#[derive(Clone, Debug, PartialEq, Eq, Copy, Hash)]
pub enum GroupingKind {
    Series,
    Anthology,
}

impl GroupingKind {
    pub fn parse(s: &str) -> Result<Self, &str> {
        TagType::parse(s)?.try_into().map_err(|_| s)
    }
}

impl From<GroupingKind> for TagType {
    fn from(value: GroupingKind) -> Self {
        Self::from(&value)
    }
}

impl<'a> From<&'a GroupingKind> for TagType {
    fn from(value: &'a GroupingKind) -> Self {
        match value {
            GroupingKind::Anthology => Self::Anthology,
            GroupingKind::Series => Self::Series,
        }
    }
}

impl TryFrom<TagType> for GroupingKind {
    type Error = TagType;

    fn try_from(value: TagType) -> Result<Self, Self::Error> {
        match value {
            TagType::Anthology => Ok(Self::Anthology),
            TagType::Series => Ok(Self::Series),
            _ => Err(value),
        }
    }
}

impl<'a> TryFrom<&'a str> for GroupingKind {
    type Error = &'a str;

    fn try_from(value: &'a str) -> Result<Self, Self::Error> {
        Self::parse(value)
    }
}

impl<'de> serde::Deserialize<'de> for GroupingKind {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        Self::parse(&s).map_err(serde::de::Error::custom)
    }
}
