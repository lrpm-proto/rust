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

impl<M, V> BasicValue<M, V> for Body<V> {
    fn ty(&self) -> BasicType {
        BasicType::Str
    }

    fn as_val(&self) -> &V {
        self.inner.as_inner()
    }

    fn into_val(self) -> V {
        self.inner.into_inner()
    }

    impl_invalid_basic_types!(<M, V> U8, U64, Str, Map);
}

impl<M, V> FromBasicValuePart<M, V> for Body<V> {
    type Error = UnexpectedType;

    fn expected_types() -> &'static [BasicType] {
        &[BasicType::Val]
    }

    fn from_basic_val(v: V) -> Result<Self, Self::Error> {
        Ok(Body::new(v))
    }
}
