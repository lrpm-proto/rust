use super::*;

/// Represents a single request unique within a session.
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct Id(u64);

impl Id {
    pub const fn new(value: u64) -> Self {
        Self(value)
    }
}

impl<M, V> BasicValue<M, V> for Id {
    fn ty(&self) -> BasicType {
        BasicType::U64
    }

    fn as_u64(&self) -> u64 {
        self.0
    }

    impl_invalid_basic_types!(<M, V> U8, Str, Map, Val);
}

impl<M, V> FromBasicValuePart<M, V> for Id {
    type Error = UnexpectedType;

    fn expected_types() -> &'static [BasicType] {
        &[BasicType::Map]
    }

    fn from_basic_u64(v: u64) -> Result<Self, Self::Error> {
        Ok(Self(v))
    }
}

impl From<Id> for u64 {
    fn from(id: Id) -> Self {
        id.0
    }
}
