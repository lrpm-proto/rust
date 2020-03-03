use std::marker::PhantomData;
//use std::collections::BTreeMap;
use std::convert::{Infallible, TryFrom};

use serde::{Deserialize, Serialize};

use super::*;

pub use bytestring::ByteString;

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

pub trait BasicValue {
    type Val;

    type Map;

    /// Returns the basic type of this basic value.
    fn ty(&self) -> BasicType;

    fn as_u8(&self) -> u8;

    fn as_u64(&self) -> u64;

    fn as_str(&self) -> &str;

    fn as_map(&self) -> &Self::Map;

    fn as_val(&self) -> &Self::Val;

    fn into_string(self) -> ByteString;

    fn into_map(self) -> Self::Map;

    fn into_val(self) -> Self::Val;
}

// impl<'a, T> BasicValue for &'a T
// where
//     T: BasicValue,
//     T::Map: Clone,
//     T::Val: Clone,
// {
//     type Map = T::Map;
//     type Val = T::Val;

//     #[inline]
//     fn ty(&self) -> BasicType {
//         (*self).ty()
//     }

//     #[inline]
//     fn as_u8(&self) -> u8 {
//         (*self).as_u8()
//     }

//     #[inline]
//     fn as_u64(&self) -> u64 {
//         (*self).as_u64()
//     }

//     #[inline]
//     fn as_str(&self) -> &str {
//         (*self).as_str()
//     }

//     #[inline]
//     fn as_map(&self) -> &Self::Map {
//         (*self).as_map()
//     }

//     #[inline]
//     fn as_val(&self) -> &Self::Val {
//         (*self).as_val()
//     }

//     #[inline]
//     fn into_string(self) -> ByteString {
//         self.as_str().into()
//     }

//     #[inline]
//     fn into_map(self) -> Self::Map {
//         self.as_map().clone()
//     }

//     #[inline]
//     fn into_val(self) -> Self::Val {
//         self.as_val().clone()
//     }
// }

pub trait BasicValueExt: BasicValue + Sized {
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

    // #[inline]
    // fn map_into<T>(self) -> Result<T, T::Error>
    // where
    //     T: FromBasicValue<Self>,
    //     Self::Map: Into<MapSource<Self::Val>>,
    // {
    //     T::from_basic_value(self)
    // }

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
    fn try_as_map(&self) -> Result<&Self::Map, UnexpectedType> {
        self.expect_types(&[BasicType::Map])?;
        Ok(self.as_map())
    }

    #[inline]
    fn try_as_val(&self) -> Result<&Self::Val, UnexpectedType> {
        self.expect_types(&[BasicType::Val])?;
        Ok(self.as_val())
    }

    #[inline]
    fn try_into_string(self) -> Result<ByteString, UnexpectedType> {
        self.expect_types(&[BasicType::Str])?;
        Ok(self.into_string())
    }

    #[inline]
    fn try_into_map(self) -> Result<Self::Map, UnexpectedType> {
        self.expect_types(&[BasicType::Map])?;
        Ok(self.into_map())
    }

    #[inline]
    fn try_into_val(self) -> Result<Self::Val, UnexpectedType> {
        self.expect_types(&[BasicType::Val])?;
        Ok(self.into_val())
    }
}

impl<T> BasicValueExt for T where T: BasicValue {}

pub trait FromBasicValue<B: BasicValue>: Sized {
    type Error: From<UnexpectedType>;

    fn expected_types() -> &'static [BasicType];

    fn from_basic_value(value: B) -> Result<Self, Self::Error>;
}

///////////////////////////////////////////////////////////////////////////////

impl BasicValue for u8 {
    type Map = InvalidBasicValue;
    type Val = InvalidBasicValue;

    #[inline]
    fn ty(&self) -> BasicType {
        BasicType::U8
    }

    #[inline]
    fn as_u8(&self) -> Self {
        *self
    }

    impl_invalid_basic_types!(U64, Str, Map, Val);
}

///////////////////////////////////////////////////////////////////////////////

impl BasicValue for u64 {
    type Map = InvalidBasicValue;
    type Val = InvalidBasicValue;

    #[inline]
    fn ty(&self) -> BasicType {
        BasicType::U64
    }

    #[inline]
    fn as_u64(&self) -> Self {
        *self
    }

    impl_invalid_basic_types!(U8, Str, Map, Val);
}

///////////////////////////////////////////////////////////////////////////////

impl BasicValue for ByteString {
    type Map = InvalidBasicValue;
    type Val = InvalidBasicValue;

    #[inline]
    fn ty(&self) -> BasicType {
        BasicType::Str
    }

    #[inline]
    fn as_str(&self) -> &str {
        self.as_ref()
    }

    #[inline]
    fn into_string(self) -> Self {
        self
    }

    impl_invalid_basic_types!(U8, U64, Map, Val);
}

///////////////////////////////////////////////////////////////////////////////

/// A wrapper around a basic `map` value.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Map<M, V> {
    inner: M,
    value: PhantomData<V>,
}

impl<M, V> Map<M, V> {
    pub fn new(inner: M) -> Self {
        Self {
            inner,
            value: PhantomData,
        }
    }
}

impl<M, V> BasicValue for Map<M, V> {
    type Map = Self;
    type Val = Val<V>;

    #[inline]
    fn ty(&self) -> BasicType {
        BasicType::Map
    }

    #[inline]
    fn as_map(&self) -> &Self {
        self
    }

    #[inline]
    fn into_map(self) -> Self {
        self
    }

    impl_invalid_basic_types!(U8, U64, Str, Val);
}

impl<M, V> From<InvalidBasicValue> for Map<M, V> {
    fn from(_: InvalidBasicValue) -> Self {
        unreachable!()
    }
}

// pub enum MapSource<V> {
//     Map(Map<V>),
//     Items(Box<dyn Iterator<Item = (ByteString, V)>>),
// }

// impl<V> From<InvalidBasicValue> for MapSource<V> {
//     fn from(_: InvalidBasicValue) -> Self {
//         unreachable!()
//     }
// }

// impl<V> From<Map<V>> for MapSource<V> {
//     fn from(map: Map<V>) -> Self {
//         MapSource::Map(map)
//     }
// }

///////////////////////////////////////////////////////////////////////////////

/// A wrapper around a basic `val` value.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Val<V> {
    inner: V,
}

impl<V> Val<V> {
    pub fn new(inner: V) -> Self {
        Self { inner }
    }

    pub fn into_inner(self) -> V {
        self.inner
    }
}

impl<V> From<InvalidBasicValue> for Val<V> {
    fn from(_: InvalidBasicValue) -> Self {
        unreachable!()
    }
}

impl<V> BasicValue for Val<V> {
    type Map = InvalidBasicValue;
    type Val = Self;

    #[inline]
    fn ty(&self) -> BasicType {
        BasicType::Map
    }

    #[inline]
    fn as_val(&self) -> &Self {
        &self
    }

    #[inline]
    fn into_val(self) -> Self {
        self
    }

    impl_invalid_basic_types!(U8, U64, Str, Map);
}

///////////////////////////////////////////////////////////////////////////////

#[derive(Debug, Clone)]
pub enum ConcreteBasicValue<M, V> {
    U8(u8),
    U64(u64),
    Str(ByteString),
    Map(Map<M, V>),
    Val(Val<V>),
}

impl<M, V> BasicValue for ConcreteBasicValue<M, V> {
    type Map = Map<M, V>;
    type Val = Val<V>;

    #[inline]
    fn ty(&self) -> BasicType {
        use ConcreteBasicValue::*;
        match self {
            U8(_) => BasicType::U8,
            U64(_) => BasicType::U64,
            Str(_) => BasicType::Str,
            Map(_) => BasicType::Map,
            Val(_) => BasicType::Val,
        }
    }

    #[inline]
    fn as_u8(&self) -> u8 {
        match self {
            Self::U8(v) => *v,
            other => panic_with_expected_type(other, BasicType::U8),
        }
    }

    #[inline]
    fn as_u64(&self) -> u64 {
        match self {
            Self::U64(v) => *v,
            other => panic_with_expected_type(other, BasicType::U64),
        }
    }

    #[inline]
    fn as_str(&self) -> &str {
        match self {
            Self::Str(v) => v.as_ref(),
            other => panic_with_expected_type(other, BasicType::Str),
        }
    }

    #[inline]
    fn as_map(&self) -> &Map<M, V> {
        match self {
            Self::Map(m) => &m,
            other => panic_with_expected_type(other, BasicType::Map),
        }
    }

    #[inline]
    fn as_val(&self) -> &Val<V> {
        match self {
            Self::Val(v) => &v,
            other => panic_with_expected_type(other, BasicType::Val),
        }
    }

    #[inline]
    fn into_string(self) -> ByteString {
        match self {
            Self::Str(v) => v,
            other => panic_with_expected_type(&other, BasicType::Str),
        }
    }

    #[inline]
    fn into_map(self) -> Map<M, V> {
        match self {
            Self::Map(m) => m,
            other => panic_with_expected_type(&other, BasicType::Map),
        }
    }

    #[inline]
    fn into_val(self) -> Val<V> {
        match self {
            Self::Val(v) => v,
            other => panic_with_expected_type(&other, BasicType::Val),
        }
    }
}

impl<B, M, V> FromBasicValue<B> for ConcreteBasicValue<M, V>
where
    B: BasicValue,
    Val<V>: From<B::Val>,
    Map<M, V>: From<B::Map>,
{
    type Error = Infallible;

    fn expected_types() -> &'static [BasicType] {
        &[
            BasicType::U8,
            BasicType::U64,
            BasicType::Str,
            BasicType::Map,
            BasicType::Val,
        ]
    }

    fn from_basic_value(value: B) -> Result<Self, Self::Error> {
        let v = match value.ty() {
            BasicType::U8 => Self::U8(value.as_u8()),
            BasicType::U64 => Self::U64(value.as_u64()),
            BasicType::Str => Self::Str(value.into_string()),
            BasicType::Map => Self::Map(value.into_map().into()),
            BasicType::Val => Self::Val(value.into_val().into()),
        };
        Ok(v)
    }
}

///////////////////////////////////////////////////////////////////////////////

/// A basic value that can never be constructed.
pub enum InvalidBasicValue {}

impl<M, V> TryFrom<Map<M, V>> for InvalidBasicValue {
    type Error = UnexpectedType;

    fn try_from(_: Map<M, V>) -> Result<Self, Self::Error> {
        Err(UnexpectedType {
            actual: BasicType::Map,
            expected: &[],
        })
    }
}

impl<V> TryFrom<Val<V>> for InvalidBasicValue {
    type Error = UnexpectedType;

    fn try_from(_: Val<V>) -> Result<Self, Self::Error> {
        Err(UnexpectedType {
            actual: BasicType::Val,
            expected: &[],
        })
    }
}
