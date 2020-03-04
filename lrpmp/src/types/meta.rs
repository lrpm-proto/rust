use serde::{Deserialize, Serialize};

use super::*;

/// An arbitrary map of additional information.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Meta<M, V> {
    inner: Map<M, V>,
}

impl<M, V> Meta<M, V> {
    pub fn new(map: M) -> Self {
        Self {
            inner: Map::new(map),
        }
    }
}

impl<M, V> Default for Meta<M, V>
where
    M: Default,
{
    fn default() -> Self {
        Meta::new(M::default())
    }
}

impl<M, V> BasicValue<M, V> for Meta<M, V> {
    fn ty(&self) -> BasicType {
        BasicType::Map
    }

    fn as_map(&self) -> &M {
        &self.inner.as_inner()
    }

    fn into_map(self) -> M {
        self.inner.into_inner()
    }

    impl_invalid_basic_types!(<M, V> U8, U64, Str, Val);
}

impl<M, V> FromBasicValuePart<M, V> for Meta<M, V> {
    type Error = UnexpectedType;

    fn expected_types() -> &'static [BasicType] {
        &[BasicType::Map]
    }

    fn from_basic_map(v: M) -> Result<Self, Self::Error> {
        Ok(Self::new(v))
    }
}
