pub mod series;
use std::ops::Deref;

pub use series::{Series, Tagging as SeriesTagging};

mod anthology;
pub use anthology::Anthology;

mod meta;
pub use meta::GroupingMeta as Meta;

mod kind;
pub use kind::GroupingKind as Kind;

mod inner;
pub use inner::GroupingInner as Inner;

pub enum Grouping {
    Anthology(Anthology),
    Series(Series),
}

impl From<Anthology> for Grouping {
    fn from(value: Anthology) -> Self {
        Self::Anthology(value)
    }
}

impl TryFrom<Grouping> for Anthology {
    type Error = Grouping;

    fn try_from(value: Grouping) -> Result<Self, Self::Error> {
        if let Grouping::Anthology(anthology) = value {
            Ok(anthology)
        } else {
            Err(value)
        }
    }
}

impl From<Series> for Grouping {
    fn from(value: Series) -> Self {
        Self::Series(value)
    }
}

impl TryFrom<Grouping> for Series {
    type Error = Grouping;

    fn try_from(value: Grouping) -> Result<Self, Self::Error> {
        if let Grouping::Series(series) = value {
            Ok(series)
        } else {
            Err(value)
        }
    }
}

impl Deref for Grouping {
    type Target = Inner;

    fn deref(&self) -> &Self::Target {
        match self {
            Self::Anthology(anthology) => anthology,
            Self::Series(series) => series,
        }
    }
}
