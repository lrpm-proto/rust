use serde::{Deserialize, Serialize};

use super::*;

/// Application specific value.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Body<V> {
    inner: Val<V>,
}

impl<V> Body<V> {
    pub fn new(val: V) -> Self {
        Self {
            inner: Val::new(val),
        }
    }
}

impl<V> BasicValue for Body<V> {
    type Map = InvalidBasicValue;
    type Val = Val<V>;

    fn ty(&self) -> BasicType {
        BasicType::Str
    }

    fn as_val(&self) -> &Val<V> {
        &self.inner
    }

    fn into_val(self) -> Val<V> {
        self.inner
    }

    impl_invalid_basic_types!(U8, U64, Str, Map);
}

impl<B, V> FromBasicValue<B> for Body<V>
where
    B: BasicValue,
    Val<V>: From<B::Val>,
{
    type Error = UnexpectedType;

    fn expected_types() -> &'static [BasicType] {
        &[BasicType::Val]
    }

    fn from_basic_value(value: B) -> Result<Self, Self::Error> {
        Ok(Self {
            inner: value.try_into_val()?.into(),
        })
    }
}
