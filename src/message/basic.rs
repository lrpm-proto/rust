use std::collections::HashMap;

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
    Str(String),
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

/// A `Str` key to `Val` structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Map<V>(HashMap<String, V>);

impl<'a, V> AsBasicValueRef<'a, V> for Map<V> {
    fn as_basic_value_ref(&'a self) -> BasicValueRef<'a, V> {
        BasicValueRef::Map(&self)
    }
}
