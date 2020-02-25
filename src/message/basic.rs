use std::collections::HashMap;
use std::convert::Infallible;

use bytestring::ByteString;
use serde::{Deserialize, Serialize};

/// Error produced from a invalid basic type conversion.
#[derive(Debug, Clone, PartialEq)]
pub struct UnexpectedBasicTypeError {
    pub expected: &'static [BasicType],
    pub actual: BasicType,
}

/// The basic types used by LRPMP.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum BasicType {
    U8,
    U64,
    Str,
    Map,
    Val,
}

/// The basic types along with a respective value used by LRPMP.
#[derive(Debug, Clone)]
pub enum BasicValue<V> {
    U8(u8),
    U64(u64),
    Str(ByteString),
    Map(Map<V>),
    Val(V),
}

impl<V> BasicValue<V> {
    /// Returns the basic type of the value.
    pub fn ty(&self) -> BasicType {
        match self {
            Self::U8(_) => BasicType::U8,
            Self::U64(_) => BasicType::U64,
            Self::Str(_) => BasicType::Str,
            Self::Map(_) => BasicType::Map,
            Self::Val(_) => BasicType::Val,
        }
    }
}

/// The basic types along with a reference to their respective
/// value used by LRPMP.
#[derive(Debug, Clone)]
pub enum BasicValueRef<'a, V> {
    U8(u8),
    U64(u64),
    Str(&'a str),
    Map(&'a Map<V>),
    Val(&'a V),
}

impl<'a, V> BasicValueRef<'a, V> {
    /// Returns the basic type of the value.
    pub fn ty(&self) -> BasicType {
        match self {
            Self::U8(_) => BasicType::U8,
            Self::U64(_) => BasicType::U64,
            Self::Str(_) => BasicType::Str,
            Self::Map(_) => BasicType::Map,
            Self::Val(_) => BasicType::Val,
        }
    }
}

/// Helper to convert special types to their basic representation.
///
/// Converting to a basic type is always a non fail operation.
pub trait AsBasicValueRef<'a, V> {
    fn as_basic_value_ref(&'a self) -> BasicValueRef<'a, V>;
}

impl<'a, T, V> AsBasicValueRef<'a, V> for &'a T
where
    T: AsBasicValueRef<'a, V>,
{
    fn as_basic_value_ref(&'a self) -> BasicValueRef<'a, V> {
        (*self).as_basic_value_ref()
    }
}

impl<'a, V> AsBasicValueRef<'a, V> for BasicValue<V> {
    fn as_basic_value_ref(&'a self) -> BasicValueRef<'a, V> {
        match self {
            Self::U8(v) => BasicValueRef::U8(*v),
            Self::U64(v) => BasicValueRef::U64(*v),
            Self::Str(s) => BasicValueRef::Str(s.as_ref()),
            Self::Map(m) => BasicValueRef::Map(&m),
            Self::Val(v) => BasicValueRef::Val(&v),
        }
    }
}

pub trait FromBasicValue<V>: Sized {
    type Error;

    fn expected_types() -> &'static [BasicType];

    fn from_basic_value(value: BasicValue<V>) -> Result<Self, Self::Error>;

    fn unexpected_error(unexpected: BasicValue<V>) -> Self::Error
    where
        Self::Error: From<UnexpectedBasicTypeError>,
    {
        UnexpectedBasicTypeError {
            expected: Self::expected_types(),
            actual: unexpected.ty(),
        }
        .into()
    }
}

impl<V> FromBasicValue<V> for BasicValue<V> {
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

    fn from_basic_value(value: BasicValue<V>) -> Result<Self, Self::Error> {
        Ok(value)
    }
}

/// A `Str` key to `Val` structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Map<V>(HashMap<String, V>);

impl<V> Map<V> {
    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    pub fn iter(&self) -> impl Iterator<Item = (&str, &V)> {
        self.0.iter().map(|(k, v)| (k.as_str(), v))
    }
}

impl<'a, V> AsBasicValueRef<'a, V> for Map<V> {
    fn as_basic_value_ref(&'a self) -> BasicValueRef<'a, V> {
        BasicValueRef::Map(&self)
    }
}
