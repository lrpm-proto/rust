use std::collections::BTreeMap;

use crate::types::{
    BasicType, BasicValue, BasicValueExt, ByteString, ConcreteBasicValue, FromBasicValue,
};

pub use serde_cbor::Value;

// let iter = src_map.into_iter().map(|(k, v)| {
//     if let Value::Text(k) = k {
//         (k, v)
//     } else {
//         unreachable!()
//     }
// });
// (iter.collect())

fn all_keys_are_string(map: &BTreeMap<Value, Value>) -> bool {
    map.iter().all(|(k, _)| match k {
        Value::Text(_) => true,
        _ => false,
    })
}

type Map = BTreeMap<ByteString, Value>;
type Val = Value;

impl BasicValue<Map, Val> for Value {
    fn ty(&self) -> BasicType {
        match self {
            Value::Integer(i) if i >= 0 && i <= u8::max_value() as i128 => {
                BasicType::U8
            },
            Value::Integer(i) if i >= 0 && i <= u64::max_value() as i128 => {
                BasicType::U64
            },
            Value::Text(t) => BasicType::Str,
            Value::Map(m) if all_keys_are_string(&m) => BasicType::Map,
            val => BasicType::Val,
        }
    }

    fn as_u8(&self) -> u8 {
        assert_basic_value_is_type(&self, BasicType::U8);
        match self {
            Value::Integer(i) => i as u8,
            _ => unreachable!(),
        }
    }

    fn as_u64(&self) -> u64 {
        assert_basic_value_is_type(&self, BasicType::U64);
        match self {
            Value::Integer(i) => i as u64,
            _ => unreachable!(),
        }
    }

    fn as_str(&self) -> &str {
        assert_basic_value_is_type(&self, BasicType::Str);
        match self {
            Value::Text(t) => t.as_ref(),
            _ => unreachable!(),
        }
    }

    fn as_map(&self) -> &M {
        assert_basic_value_is_type(&self, BasicType::Map);
        match self {
            Value::Map(t) => t.as_ref(),
            _ => unreachable!(),
        }
    }

    fn as_val(&self) -> &V;

    fn into_string(self) -> ByteString;

    fn into_map(self) -> M;

    fn into_val(self) -> V;
}

// impl IntoBasicValue for Value {
//     type Value = ConcreteBasicValue<Map, Val>;

//     fn into_basic_value(self) -> Self::Value {
//         match self {
//             Value::Integer(i) if i >= 0 && i <= u8::max_value() as i128 => {
//                 ConcreteBasicValue::U8(i as u8)
//             }
//             Value::Integer(i) if i >= 0 && i <= u64::max_value() as i128 => {
//                 ConcreteBasicValue::U64(i as u64)
//             }
//             Value::Text(t) => ConcreteBasicValue::Str(t.into()),
//             Value::Map(src_map) if all_keys_are_string(&src_map) => {
//                 let iter = src_map.into_iter().map(|(k, v)| {
//                     if let Value::Text(k) = k {
//                         (k, v)
//                     } else {
//                         unreachable!()
//                     }
//                 });
//                 ConcreteBasicValue::Map(iter.collect())
//             }
//             val => ConcreteBasicValue::Val(val),
//         }
//     }
// }
