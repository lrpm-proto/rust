use std::collections::{BTreeMap, HashMap};
use std::ops::{Deref, DerefMut};

use super::json;
use crate::types;

pub type Meta<V> = types::Meta<Map<V>, V>;

type MapInner<V> = BTreeMap<String, V>;

#[derive(Debug)]
pub struct Map<V> {
    inner: MapInner<V>,
}

impl<V> From<BTreeMap<String, V>> for Map<V> {
    fn from(map: BTreeMap<String, V>) -> Self {
        Self { inner: map }
    }
}

impl<V> From<HashMap<String, V>> for Map<V> {
    fn from(map: HashMap<String, V>) -> Self {
        Self {
            inner: map.into_iter().collect(),
        }
    }
}

impl From<json::Map> for Map<json::Value> {
    fn from(map: json::Map) -> Self {
        Self {
            inner: map.into_iter().collect(),
        }
    }
}

impl<V> Deref for Map<V> {
    type Target = MapInner<V>;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl<V> DerefMut for Map<V> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl<V> Default for Map<V> {
    fn default() -> Self {
        Self {
            inner: Default::default(),
        }
    }
}
