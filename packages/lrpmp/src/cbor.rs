use std::collections::BTreeMap;

pub use serde_cbor::Value;

use serde::{Deserialize, Serialize};

use serde_cbor::de::{Deserializer, IoRead};
use serde_cbor::ser::{IoWrite, Serializer};
use serde_cbor::Error as InnerError;

use crate::io::{Read, Write};
use crate::message::basic::BasicValue;
use crate::message::special::KnownKind;
use crate::message::{self as msg, Message, MessageError};
use crate::serde::{ArrayDecoder, ArrayEncoder, ArrayFieldDecoder, ArrayFieldEncoder};

pub type Error = MessageError<InnerError>;

pub struct MessageEncoder<W: Write> {
    inner: Serializer<IoWrite<W>>,
}

impl<W: Write> MessageEncoder<W> {
    pub fn from_writer(writer: W) -> Self {
        Self {
            inner: Serializer::new(IoWrite::new(writer)),
        }
    }
}

impl<'a, V, W> msg::MessageEncoder<V> for &'a mut MessageEncoder<W>
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

pub struct MessageDecoder<R: Read> {
    inner: Deserializer<IoRead<R>>,
}

impl<R: Read> MessageDecoder<R> {
    pub fn from_reader(reader: R) -> Self {
        Self {
            inner: Deserializer::new(IoRead::new(reader)),
        }
    }
}

impl<'a, V, R> msg::MessageDecoder<V> for &'a mut MessageDecoder<R>
where
    R: Read,
    V: Deserialize<'a> + Into<BasicValue<V>>,
{
    type Error = InnerError;
    type FieldDecoder = ArrayFieldDecoder<V, InnerError>;

    fn start(self) -> Result<(KnownKind, Self::FieldDecoder), MessageError<Self::Error>> {
        msg::MessageDecoder::<V>::start(ArrayDecoder::new(&mut self.inner))
    }
}

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

#[cfg(test)]
mod tests {
    use bytes::buf::{BufExt, BufMutExt};
    use bytes::BytesMut;

    use super::*;
    use crate::message::special::{Body, Meta};
    use crate::message::{HelloMessage, StandardMessage};

    #[test]
    fn test_message_encoder_decoder() {
        let src_message = HelloMessage::new(Body::new(Value::Text("1".into())), Meta::new());
        // Encoder
        let mut writer = BytesMut::new().writer();
        let mut encoder = MessageEncoder::from_writer(&mut writer);
        src_message.encode(&mut encoder).unwrap();
        // Buf
        let buf = writer.into_inner();
        assert_eq!(&[0x83, 0x02, 0x61, 0x31, 0xA0][..], &buf[..]);
        // Decoder
        let reader = buf.reader();
        let mut decoder = MessageDecoder::from_reader(reader);
        let message = StandardMessage::<Value>::decode(&mut decoder).unwrap();
        match message {
            StandardMessage::Hello(_) => (),
            other => panic!("unexpected message {:?}", other),
        }
    }
}
