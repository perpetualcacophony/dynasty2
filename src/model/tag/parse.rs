use super::{TagInternal, Type};

pub trait ParseTag {
    const TYPE: Type;

    fn parse(tag: TagInternal) -> Option<Self>
    where
        Self: Sized,
    {
        (tag.type_ == Self::TYPE).then_some(Self::from_tag(tag))
    }

    fn from_tag(tag: TagInternal) -> Self
    where
        Self: Sized;
}
