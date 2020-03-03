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

impl<M, V> BasicValue for Meta<M, V> {
    type Map = Map<M, V>;
    type Val = Val<V>;

    fn ty(&self) -> BasicType {
        BasicType::Map
    }

    fn as_map(&self) -> &Map<M, V> {
        &self.inner
    }

    fn into_map(self) -> Map<M, V> {
        self.inner
    }

    impl_invalid_basic_types!(U8, U64, Str, Val);
}

impl<B, M, V> FromBasicValue<B> for Meta<M, V>
where
    B: BasicValue<Map = Map<M, V>>,
{
    type Error = UnexpectedType;

    fn expected_types() -> &'static [BasicType] {
        &[BasicType::Map]
    }

    fn from_basic_value(value: B) -> Result<Self, Self::Error> {
        // let map = match value.try_into_map()?.into() {
        //     MapSource::Map(m) => m,
        //     MapSource::Items(i) => i.collect(),
        // };

        Ok(Self {
            inner: value.try_into_map()?,
        })
    }
}
