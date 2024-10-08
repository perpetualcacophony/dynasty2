#[derive(Clone, Debug, PartialEq, Eq, Copy, Hash)]
pub enum TagType {
    Author,
    Anthology,
    General,
    Series,
    Scanlator,
    Pairing,
    Doujin,
    Status,
    Issue,
}

impl TagType {
    pub fn parse(s: &str) -> Result<Self, &str> {
        match s {
            "Author" => Ok(Self::Author),
            "General" => Ok(Self::General),
            "Series" => Ok(Self::Series),
            "Scanlator" => Ok(Self::Scanlator),
            "Pairing" => Ok(Self::Pairing),
            "Doujin" => Ok(Self::Doujin),
            "Anthology" => Ok(Self::Anthology),
            "Status" => Ok(Self::Status),
            "Issue" => Ok(Self::Issue),
            _ => Err(s),
        }
    }
}

impl<'a> TryFrom<&'a str> for TagType {
    type Error = &'a str;

    fn try_from(value: &'a str) -> Result<Self, Self::Error> {
        Self::parse(value)
    }
}

impl<'de> serde::Deserialize<'de> for TagType {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        Self::parse(&s).map_err(serde::de::Error::custom)
    }
}
