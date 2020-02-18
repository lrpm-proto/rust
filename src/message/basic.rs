use std::collections::HashMap;

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone)]
pub enum BasicTypeRef<'a, V> {
    U8(u8),
    U64(u64),
    Str(&'a str),
    Map(&'a Map<V>),
    Val(&'a V),
}

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
