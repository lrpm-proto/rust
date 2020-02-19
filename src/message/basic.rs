use std::collections::HashMap;

use serde::{Deserialize, Serialize};

/// The basic types used by LRPMP
#[derive(Debug, Clone)]
pub enum BasicType<V> {
    U8(u8),
    U64(u64),
    Str(String),
    Map(Map<V>),
    Val(V),
}

/// References to the basic types used by LRPMP
#[derive(Debug, Clone)]
pub enum BasicTypeRef<'a, V> {
    U8(u8),
    U64(u64),
    Str(&'a str),
    Map(&'a Map<V>),
    Val(&'a V),
}

/// Helper to convert special types to their basic representation.
///
/// Converting to a basic type is always a non fail operation.
pub trait AsBasicTypeRef<'a, V> {
    fn as_basic_type_ref(&'a self) -> BasicTypeRef<'a, V>;
}

impl<'a, T, V> AsBasicTypeRef<'a, V> for &'a T
where
    T: AsBasicTypeRef<'a, V>,
{
    fn as_basic_type_ref(&'a self) -> BasicTypeRef<'a, V> {
        (*self).as_basic_type_ref()
    }
}

impl<'a, V> AsBasicTypeRef<'a, V> for BasicType<V> {
    fn as_basic_type_ref(&'a self) -> BasicTypeRef<'a, V> {
        match self {
            Self::U8(v) => BasicTypeRef::U8(*v),
            Self::U64(v) => BasicTypeRef::U64(*v),
            Self::Str(s) => BasicTypeRef::Str(s.as_ref()),
            Self::Map(m) => BasicTypeRef::Map(&m),
            Self::Val(v) => BasicTypeRef::Val(&v),
        }
    }
}

/// A `Str` key to `Val` structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Map<V>(HashMap<String, V>);

impl<'a, V> AsBasicTypeRef<'a, V> for Map<V> {
    fn as_basic_type_ref(&'a self) -> BasicTypeRef<'a, V> {
        BasicTypeRef::Map(&self)
    }
}
