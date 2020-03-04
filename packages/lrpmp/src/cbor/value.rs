use std::collections::BTreeMap;

use crate::types::{BasicValue, ByteString, FromBasicValuePart, IntoBasicValue};

pub use serde_cbor::Value;

pub type Map = BTreeMap<ByteString, Val>;
pub type Val = Value;

fn all_keys_are_string(map: &BTreeMap<Value, Value>) -> bool {
    map.iter().all(|(k, _)| match k {
        Value::Text(_) => true,
        _ => false,
    })
}

impl<B> IntoBasicValue<B, Map, Val> for Value
where
    B: BasicValue<Map, Val>,
    B: FromBasicValuePart<Map, Val>,
{
    type Error = B::Error;

    fn into_basic(self) -> Result<B, Self::Error> {
        match self {
            Value::Integer(i) if i >= 0 && i <= u8::max_value() as i128 => {
                B::from_basic_u8(i as u8)
            }
            Value::Integer(i) if i >= 0 && i <= u64::max_value() as i128 => {
                B::from_basic_u64(i as u64)
            }
            Value::Text(t) => B::from_basic_str(t.into()),
            Value::Map(src_map) if all_keys_are_string(&src_map) => {
                let iter = src_map.into_iter().map(|(k, v)| {
                    if let Value::Text(k) = k {
                        (k.into(), v)
                    } else {
                        unreachable!()
                    }
                });
                B::from_basic_map(iter.collect())
            }
            val => B::from_basic_val(val),
        }
    }
}
