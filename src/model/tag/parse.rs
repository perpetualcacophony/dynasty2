use super::{Tag, Type};

pub trait ParseTag {
    const TYPE: Type;

    fn parse(tag: Tag) -> Option<Self>
    where
        Self: Sized,
    {
        (tag.type_ == Self::TYPE).then_some(Self::from_tag(tag))
    }

    fn from_tag(tag: Tag) -> Self
    where
        Self: Sized;
}
