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

/// A `Str` key to `Val` structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Map<V>(HashMap<String, V>);

impl<'a, V> AsBasicTypeRef<'a, V> for Map<V> {
    fn as_basic_type_ref(&'a self) -> BasicTypeRef<'a, V> {
        BasicTypeRef::Map(&self)
    }
}
