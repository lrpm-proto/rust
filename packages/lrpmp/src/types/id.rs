use super::*;

/// Represents a single request unique within a session.
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct Id(u64);

impl Id {
    pub const fn new(value: u64) -> Self {
        Self(value)
    }
}

impl BasicValue for Id {
    type Map = InvalidBasicValue;
    type Val = InvalidBasicValue;

    fn ty(&self) -> BasicType {
        BasicType::U64
    }

    fn as_u64(&self) -> u64 {
        self.0
    }

    impl_invalid_basic_types!(U8, Str, Map, Val);
}

impl<B> FromBasicValue<B> for Id
where
    B: BasicValue,
{
    type Error = UnexpectedType;

    fn expected_types() -> &'static [BasicType] {
        &[BasicType::Map]
    }

    fn from_basic_value(value: B) -> Result<Self, Self::Error> {
        Ok(Self(value.try_as_u64()?))
    }
}

impl From<Id> for u64 {
    fn from(id: Id) -> Self {
        id.0
    }
}
