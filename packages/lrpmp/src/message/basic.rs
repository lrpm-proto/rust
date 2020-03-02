use bytestring::ByteString;
use std::collections::BTreeMap;

pub(crate) fn expected_type<B, V, M>(got: &B, expected: BasicType) -> !
where
    B: BasicValue<V, M>,
{
    panic!("expected {:?}, got {:?}", expected, got.ty())
}

/// Error produced from a invalid basic type conversion.
#[derive(Debug, Clone, PartialEq)]
pub struct UnexpectedType {
    pub expected: &'static [BasicType],
    pub actual: BasicType,
}

/// The basic types used by LRPMP.
#[derive(Debug, Hash, Clone, Copy, PartialEq)]
pub enum BasicType {
    /// A `u8` LRPMP type.
    U8,
    /// A `u64` LRPMP type.
    U64,
    /// A `str` LRPMP type.
    Str,
    /// A `map` LRPMP type.
    Map,
    /// A `val` LRPMP type.
    Val,
}

pub trait BasicValue<V, M = Map<V>> {
    /// Returns the basic type of this basic value.
    fn ty(&self) -> BasicType;

    fn as_u8(&self) -> u8;

    fn as_u64(&self) -> u64;

    fn as_str(&self) -> &str;

    fn as_map(&self) -> &M;

    fn as_val(&self) -> &V;

    fn into_string(self) -> ByteString;

    fn into_map(self) -> M;

    fn into_val(self) -> V;
}

impl<'a, T, V, M> BasicValue<V, M> for &'a T
where
    T: BasicValue<V, M>,
    V: Clone,
    M: Clone,
{
    #[inline]
    fn ty(&self) -> BasicType {
        (*self).ty()
    }

    #[inline]
    fn as_u8(&self) -> u8 {
        (*self).as_u8()
    }

    #[inline]
    fn as_u64(&self) -> u64 {
        (*self).as_u64()
    }

    #[inline]
    fn as_str(&self) -> &str {
        (*self).as_str()
    }

    #[inline]
    fn as_map(&self) -> &M {
        (*self).as_map()
    }

    #[inline]
    fn as_val(&self) -> &V {
        (*self).as_val()
    }

    #[inline]
    fn into_string(self) -> ByteString {
        self.as_str().into()
    }

    #[inline]
    fn into_map(self) -> M {
        self.as_map().clone()
    }

    #[inline]
    fn into_val(self) -> V {
        self.as_val().clone()
    }
}

pub trait BasicValueExt<V, M>: BasicValue<V, M> + Sized {
    #[inline]
    fn expect_types(&self, expected: &'static [BasicType]) -> Result<(), UnexpectedType> {
        if expected.contains(&self.ty()) {
            Ok(())
        } else {
            Err(UnexpectedType {
                actual: self.ty(),
                expected,
            })
        }
    }

    #[inline]
    fn try_as_u8(&self) -> Result<u8, UnexpectedType> {
        self.expect_types(&[BasicType::U8])?;
        Ok(self.as_u8())
    }

    #[inline]
    fn try_as_u64(&self) -> Result<u64, UnexpectedType> {
        self.expect_types(&[BasicType::U64])?;
        Ok(self.as_u64())
    }

    #[inline]
    fn try_as_str(&self) -> Result<&str, UnexpectedType> {
        self.expect_types(&[BasicType::Str])?;
        Ok(self.as_str())
    }

    #[inline]
    fn try_as_map(&self) -> Result<&M, UnexpectedType> {
        self.expect_types(&[BasicType::Map])?;
        Ok(self.as_map())
    }

    #[inline]
    fn try_as_val(&self) -> Result<&V, UnexpectedType> {
        self.expect_types(&[BasicType::Val])?;
        Ok(self.as_val())
    }

    #[inline]
    fn try_into_string(self) -> Result<ByteString, UnexpectedType> {
        self.expect_types(&[BasicType::Str])?;
        Ok(self.into_string())
    }

    #[inline]
    fn try_into_map(self) -> Result<M, UnexpectedType> {
        self.expect_types(&[BasicType::Map])?;
        Ok(self.into_map())
    }

    #[inline]
    fn try_into_val(self) -> Result<V, UnexpectedType> {
        self.expect_types(&[BasicType::Val])?;
        Ok(self.into_val())
    }
}

impl<T, V, M> BasicValueExt<V, M> for T where T: BasicValue<V, M> {}

macro_rules! impl_invalid_basic_types {
    (<$V:ty, $M:ty>, $($ty:ident),*) => {
        $(
            impl_invalid_basic_types!($ty, $V, $M);
        )*
    };
    (U8, $V:ty, $M:ty) => {
        #[inline]
        fn as_u8(&self) -> u8 {
            expected_type::<Self, $V, $M>(&self, BasicType::U8)
        }
    };
    (U64, $V:ty, $M:ty) => {
        #[inline]
        fn as_u64(&self) -> u64 {
            expected_type::<Self, $V, $M>(&self, BasicType::U64)
        }
    };
    (Str, $V:ty, $M:ty) => {
        #[inline]
        fn as_str(&self) -> &str {
            expected_type::<Self, $V, $M>(&self, BasicType::Str)
        }

        #[inline]
        fn into_string(self) -> ByteString {
            expected_type::<Self, $V, $M>(&self, BasicType::Str)
        }
    };
    (Map, $V:ty, $M:ty) => {
        #[inline]
        fn as_map(&self) -> &M {
            expected_type::<Self, $V, $M>(&self, BasicType::Map)
        }

        #[inline]
        fn into_map(self) -> M {
            expected_type::<Self, $V, $M>(&self, BasicType::Map)
        }
    };
    (Val, $V:ty, $M:ty) => {
        #[inline]
        fn as_val(&self) -> &V {
            expected_type::<Self, $V, $M>(&self, BasicType::Val)
        }

        #[inline]
        fn into_val(self) -> V {
            expected_type::<Self, $V, $M>(&self, BasicType::Val)
        }
    };
}

impl<V, M> BasicValue<V, M> for u8 {
    #[inline]
    fn ty(&self) -> BasicType {
        BasicType::U8
    }

    #[inline]
    fn as_u8(&self) -> u8 {
        *self
    }

    impl_invalid_basic_types!(<V, M>, U64, Str, Map, Val);
}

impl<V, M> BasicValue<V, M> for u64 {
    #[inline]
    fn ty(&self) -> BasicType {
        BasicType::U64
    }

    #[inline]
    fn as_u64(&self) -> u64 {
        *self
    }

    impl_invalid_basic_types!(<V, M>, U8, Str, Map, Val);
}

impl<V, M> BasicValue<V, M> for ByteString {
    #[inline]
    fn ty(&self) -> BasicType {
        BasicType::Str
    }

    #[inline]
    fn as_str(&self) -> &str {
        self.as_ref()
    }

    #[inline]
    fn into_string(self) -> ByteString {
        self
    }

    impl_invalid_basic_types!(<V, M>, U8, U64, Map, Val);
}

pub type Map<V> = BTreeMap<ByteString, V>;

impl<V> BasicValue<V, Map<V>> for Map<V> {
    #[inline]
    fn ty(&self) -> BasicType {
        BasicType::Map
    }

    #[inline]
    fn as_map(&self) -> &Map<V> {
        self
    }

    #[inline]
    fn into_map(self) -> Map<V> {
        self
    }

    impl_invalid_basic_types!(<V, Map<V>>, U8, U64, Str, Val);
}

pub trait IntoBasicValue<V>: Sized {
    type BasicValue: BasicValue<V>;

    fn into_basic_value(self) -> Self::BasicValue;
}

pub trait FromBasicValue<V>: Sized {
    type Error: From<UnexpectedType>;

    fn expected_types() -> &'static [BasicType];

    fn from_basic_value<B>(value: B) -> Result<Self, Self::Error>
    where
        B: BasicValue<V>;
}
