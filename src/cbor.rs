use std::collections::BTreeMap;
use std::marker::PhantomData;

pub use serde_cbor::Value;

use serde::Serialize;

use serde_cbor::de::{Deserializer, IoRead};
use serde_cbor::ser::{IoWrite, Serializer};
use serde_cbor::Error as InnerError;

use crate::io::{Read, Write};
use crate::message::basic::BasicValue;
use crate::message::special::KnownKind;
use crate::message::{self as msg, Message, MessageError};
use crate::serde::{ArrayDecoder, ArrayFieldEncoder, ArrayEncoder};

pub type Error = MessageError<InnerError>;

pub struct MessageEncoder<'a, W: Write> {
    inner: Serializer<IoWrite<W>>,
    lifetime: PhantomData::<&'a ()>,
}

impl<'a, W: Write> MessageEncoder<'a, W> {
    pub fn from_writer(writer: W) -> Self {
        Self {
            inner: Serializer::new(IoWrite::new(writer)),
            lifetime: PhantomData,
        }
    }
}

impl<'a, V, W: 'a> msg::MessageEncoder<V> for MessageEncoder<'a, W>
where
    W: Write,
    V: Serialize,
{
    type Ok = (); 
    type Error = InnerError;
    type FieldEncoder = ArrayFieldEncoder<&'a mut Serializer<IoWrite<W>>>;

    fn start(self, kind: KnownKind) -> Result<Self::FieldEncoder, MessageError<Self::Error>> {
        msg::MessageEncoder::<V>::start(ArrayEncoder::new(&mut self.inner), kind)
    }
}
//
// pub type MessageDecoder<'de, R> = ArrayDecoder<'de, Deserializer<R>>;

// impl<'de, R: Read> MessageDecoder<'de, IoRead<R>> {
//     pub fn from_reader(reader: R) -> Self {
//         let inner = Deserializer::new(IoRead::new(reader));
//         ArrayDecoder::new(inner)
//     }
// } 

pub struct MessageWriter<W: Write> {
    inner: Serializer<IoWrite<W>>,
}

impl<W> msg::MessageWriter<W> for MessageWriter<W>
where
    W: Write,
{
    type Value = Value;
    type Error = Error;

    fn write_message<M>(&mut self, message: &M) -> Result<(), Self::Error>
    where
        M: Message<Self::Value>,
    {
        message.encode_ref(ArrayEncoder::new(&mut self.inner))
    }
}

pub struct MessageReader<R>
where
    R: Read,
{
    inner: Deserializer<IoRead<R>>,
}

impl<R> msg::MessageReader<R> for MessageReader<R>
where
    R: Read,
{
    type Value = Value;
    type Error = Error;

    fn read_message<M>(&mut self) -> Result<M, Self::Error>
    where
        M: Message<Self::Value>,
    {
        M::decode(ArrayDecoder::new(&mut self.inner))
    }
}

// impl From<BasicValue<Value>> for Value {
//     fn from(value: BasicValue<Value>) -> Self {
//     }
// }

impl From<Value> for BasicValue<Value> {
    fn from(value: Value) -> Self {
        fn all_keys_are_string(map: &BTreeMap<Value, Value>) -> bool {
            map.iter().all(|(k, _)| match k {
                Value::Text(_) => true,
                _ => false,
            })
        }

        match value {
            Value::Integer(i) if i >= 0 && i <= u8::max_value() as i128 => Self::U8(i as u8),
            Value::Integer(i) if i >= 0 && i <= u64::max_value() as i128 => Self::U64(i as u64),
            Value::Text(t) if t.is_ascii() => Self::Str(t.into()),
            Value::Map(src_map) if all_keys_are_string(&src_map) => {
                let iter = src_map.into_iter().map(|(k, v)| {
                    if let Value::Text(k) = k {
                        (k, v)
                    } else {
                        unreachable!()
                    }
                });
                BasicValue::Map(iter.collect())
            }
            val => BasicValue::Val(val),
        }
    }
}

// pub struct MessageTranslation;

// impl<V> msg::MessageTranslation<V> for MessageTranslation
// where
//     V: Serialize,
// {
//     type Value = Value;
//     type Error = Error;

//     fn translate<I, O>(message: I) -> Result<O, Self::Error>
//     where
//         I: Message<V>,
//         O: Message<Self::Value>
//     {
//         SerdeTranslator.translate(message)
//     }
// }
