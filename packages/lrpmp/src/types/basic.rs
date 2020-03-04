use std::convert::Infallible;
use std::marker::PhantomData;

use serde::{Deserialize, Serialize};

use super::*;

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

pub trait BasicValue<M, V> {
    /// Returns the basic type of this basic value.
    fn ty(&self) -> BasicType;

    fn as_u8(&self) -> u8;

    fn as_u64(&self) -> u64;

    fn as_str(&self) -> &str;

    fn as_map(&self) -> &M;

    fn as_val(&self) -> &V;

    fn into_string(self) -> String;

    fn into_map(self) -> M;

    fn into_val(self) -> V;
}

impl<'a, T, M, V> BasicValue<M, V> for &'a T
where
    T: BasicValue<M, V>,
    M: Clone,
    V: Clone,
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
    fn into_string(self) -> String {
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

pub trait BasicValueExt<M, V>: BasicValue<M, V> + Sized {
    #[inline]
    fn assert_is_type(&self, ty: BasicType) {
        assert_eq!(self.ty(), ty);
    }

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
    fn into_concrete(self) -> ConcreteBasicValue<M, V> {
        ConcreteBasicValue::from_basic(self).unwrap()
    }

    #[inline]
    fn map_into<T, MO, VO>(self) -> Result<T, T::Error>
    where
        MO: From<M>,
        VO: From<V>,
        T: FromBasicValuePart<MO, VO>,
    {
        match self.ty() {
            BasicType::U8 => T::from_basic_u8(self.as_u8()),
            BasicType::U64 => T::from_basic_u64(self.as_u64()),
            BasicType::Str => T::from_basic_str(self.into_string()),
            BasicType::Map => T::from_basic_map(self.into_map().into()),
            BasicType::Val => T::from_basic_val(self.into_val().into()),
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
    fn try_into_string(self) -> Result<String, UnexpectedType> {
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

impl<T, M, V> BasicValueExt<M, V> for T where T: BasicValue<M, V> {}

///////////////////////////////////////////////////////////////////////////////

pub trait FromBasicValuePart<M, V>: Sized {
    type Error: From<UnexpectedType>;

    fn expected_types() -> &'static [BasicType];

    fn from_basic_u8(v: u8) -> Result<Self, Self::Error> {
        let _ = v;
        Err(UnexpectedType {
            actual: BasicType::U8,
            expected: Self::expected_types(),
        }
        .into())
    }

    fn from_basic_u64(v: u64) -> Result<Self, Self::Error> {
        let _ = v;
        Err(UnexpectedType {
            actual: BasicType::U64,
            expected: Self::expected_types(),
        }
        .into())
    }

    fn from_basic_str(v: String) -> Result<Self, Self::Error> {
        let _ = v;
        Err(UnexpectedType {
            actual: BasicType::Str,
            expected: Self::expected_types(),
        }
        .into())
    }

    fn from_basic_map(v: M) -> Result<Self, Self::Error> {
        let _ = v;
        Err(UnexpectedType {
            actual: BasicType::Map,
            expected: Self::expected_types(),
        }
        .into())
    }

    fn from_basic_val(v: V) -> Result<Self, Self::Error> {
        let _ = v;
        Err(UnexpectedType {
            actual: BasicType::Val,
            expected: Self::expected_types(),
        }
        .into())
    }
}

///////////////////////////////////////////////////////////////////////////////

pub trait FromBasicValue<B, M, V>: Sized
where
    B: BasicValue<M, V>,
{
    type Error: From<UnexpectedType>;

    fn expected_types() -> &'static [BasicType];

    fn from_basic(value: B) -> Result<Self, Self::Error>;
}

impl<T, B, M, V> FromBasicValue<B, M, V> for T
where
    B: BasicValue<M, V>,
    T: FromBasicValuePart<M, V>,
{
    type Error = T::Error;

    #[inline]
    fn expected_types() -> &'static [BasicType] {
        T::expected_types()
    }

    fn from_basic(value: B) -> Result<Self, Self::Error> {
        match value.ty() {
            BasicType::U8 => T::from_basic_u8(value.as_u8()),
            BasicType::U64 => T::from_basic_u64(value.as_u64()),
            BasicType::Str => T::from_basic_str(value.into_string()),
            BasicType::Map => T::from_basic_map(value.into_map()),
            BasicType::Val => T::from_basic_val(value.into_val()),
        }
    }
}

///////////////////////////////////////////////////////////////////////////////

pub trait IntoBasicValue<B, M, V>
where
    B: BasicValue<M, V>,
{
    type Error: From<UnexpectedType>;

    fn into_basic(self) -> Result<B, Self::Error>;
}

impl<T, B, M, V> IntoBasicValue<B, M, V> for T
where
    T: BasicValue<M, V>,
    B: BasicValue<M, V>,
    B: FromBasicValue<T, M, V>,
{
    type Error = B::Error;

    fn into_basic(self) -> Result<B, Self::Error> {
        B::from_basic(self)
    }
}

///////////////////////////////////////////////////////////////////////////////

impl<M, V> BasicValue<M, V> for u8 {
    #[inline]
    fn ty(&self) -> BasicType {
        BasicType::U8
    }

    #[inline]
    fn as_u8(&self) -> Self {
        *self
    }

    impl_invalid_basic_types!(<M, V> U64, Str, Map, Val);
}

impl<M, V> FromBasicValuePart<M, V> for u8 {
    type Error = UnexpectedType;

    fn expected_types() -> &'static [BasicType] {
        &[BasicType::U8]
    }

    fn from_basic_u8(v: u8) -> Result<Self, Self::Error> {
        Ok(v)
    }
}

///////////////////////////////////////////////////////////////////////////////

impl<M, V> BasicValue<M, V> for u64 {
    #[inline]
    fn ty(&self) -> BasicType {
        BasicType::U64
    }

    #[inline]
    fn as_u64(&self) -> Self {
        *self
    }

    impl_invalid_basic_types!(<M, V> U8, Str, Map, Val);
}

impl<M, V> FromBasicValuePart<M, V> for u64 {
    type Error = UnexpectedType;

    fn expected_types() -> &'static [BasicType] {
        &[BasicType::U64]
    }

    fn from_basic_u64(v: u64) -> Result<Self, Self::Error> {
        Ok(v)
    }
}

///////////////////////////////////////////////////////////////////////////////

impl<M, V> BasicValue<M, V> for String {
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

    impl_invalid_basic_types!(<M, V> U8, U64, Map, Val);
}

impl<M, V> FromBasicValuePart<M, V> for String {
    type Error = UnexpectedType;

    fn expected_types() -> &'static [BasicType] {
        &[BasicType::Str]
    }

    fn from_basic_str(v: String) -> Result<Self, Self::Error> {
        Ok(v)
    }
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

    pub fn as_inner(&self) -> &M {
        &self.inner
    }

    pub fn into_inner(self) -> M {
        self.inner
    }
}

impl<M, V> BasicValue<M, V> for Map<M, V> {
    #[inline]
    fn ty(&self) -> BasicType {
        BasicType::Map
    }

    #[inline]
    fn as_map(&self) -> &M {
        self.as_inner()
    }

    #[inline]
    fn into_map(self) -> M {
        self.into_inner()
    }

    impl_invalid_basic_types!(<M, V> U8, U64, Str, Val);
}

impl<M, V> FromBasicValuePart<M, V> for Map<M, V> {
    type Error = UnexpectedType;

    fn expected_types() -> &'static [BasicType] {
        &[BasicType::Map]
    }

    fn from_basic_map(v: M) -> Result<Self, Self::Error> {
        Ok(Map::new(v))
    }
}

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

    pub fn as_inner(&self) -> &V {
        &self.inner
    }

    pub fn into_inner(self) -> V {
        self.inner
    }
}

impl<M, V> BasicValue<M, V> for Val<V> {
    #[inline]
    fn ty(&self) -> BasicType {
        BasicType::Map
    }

    #[inline]
    fn as_val(&self) -> &V {
        self.as_inner()
    }

    #[inline]
    fn into_val(self) -> V {
        self.into_inner()
    }

    impl_invalid_basic_types!(<M, V> U8, U64, Str, Map);
}

impl<M, V> FromBasicValuePart<M, V> for Val<V> {
    type Error = UnexpectedType;

    fn expected_types() -> &'static [BasicType] {
        &[BasicType::Val]
    }

    fn from_basic_val(v: V) -> Result<Self, Self::Error> {
        Ok(Val::new(v))
    }
}

///////////////////////////////////////////////////////////////////////////////

#[derive(Debug, Clone)]
pub enum ConcreteBasicValue<M, V> {
    U8(u8),
    U64(u64),
    Str(String),
    Map(M),
    Val(V),
}

impl<M, V> BasicValue<M, V> for ConcreteBasicValue<M, V> {
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
        assert_eq!(self.ty(), BasicType::U8);
        match self {
            Self::U8(v) => *v,
            _ => unreachable!(),
        }
    }

    #[inline]
    fn as_u64(&self) -> u64 {
        assert_eq!(self.ty(), BasicType::U64);
        match self {
            Self::U64(v) => *v,
            _ => unreachable!(),
        }
    }

    #[inline]
    fn as_str(&self) -> &str {
        assert_eq!(self.ty(), BasicType::Str);
        match self {
            Self::Str(v) => v.as_ref(),
            _ => unreachable!(),
        }
    }

    #[inline]
    fn as_map(&self) -> &M {
        assert_eq!(self.ty(), BasicType::Map);
        match self {
            Self::Map(m) => &m,
            _ => unreachable!(),
        }
    }

    #[inline]
    fn as_val(&self) -> &V {
        assert_eq!(self.ty(), BasicType::Val);
        match self {
            Self::Val(v) => &v,
            _ => unreachable!(),
        }
    }

    #[inline]
    fn into_string(self) -> String {
        assert_eq!(self.ty(), BasicType::Str);
        match self {
            Self::Str(v) => v,
            _ => unreachable!(),
        }
    }

    #[inline]
    fn into_map(self) -> M {
        assert_eq!(self.ty(), BasicType::Map);
        match self {
            Self::Map(m) => m,
            _ => unreachable!(),
        }
    }

    #[inline]
    fn into_val(self) -> V {
        assert_eq!(self.ty(), BasicType::Val);
        match self {
            Self::Val(v) => v,
            _ => unreachable!(),
        }
    }
}

impl<M, V> FromBasicValuePart<M, V> for ConcreteBasicValue<M, V> {
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

    #[inline]
    fn from_basic_u8(v: u8) -> Result<Self, Self::Error> {
        Ok(Self::U8(v))
    }

    #[inline]
    fn from_basic_u64(v: u64) -> Result<Self, Self::Error> {
        Ok(Self::U64(v))
    }

    #[inline]
    fn from_basic_str(v: String) -> Result<Self, Self::Error> {
        Ok(Self::Str(v))
    }

    #[inline]
    fn from_basic_map(v: M) -> Result<Self, Self::Error> {
        Ok(Self::Map(v))
    }

    #[inline]
    fn from_basic_val(v: V) -> Result<Self, Self::Error> {
        Ok(Self::Val(v))
    }
}
