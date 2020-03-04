mod value;

use serde::{Deserialize, Serialize};

use serde_cbor::de::{Deserializer, IoRead};
use serde_cbor::ser::{IoWrite, Serializer};
use serde_cbor::Error as InnerError;

use crate::io::{Read, Write};
use crate::message::{self as msg, Message, MessageError};
use crate::serde::{ArrayDecoder, ArrayEncoder, ArrayFieldDecoder, ArrayFieldEncoder};
use crate::types::{ConcreteBasicValue, IntoBasicValue, KnownKind};

pub use self::value::*;

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

impl<'a, M, V, W> msg::MessageEncoder<M, V> for &'a mut MessageEncoder<W>
where
    W: Write,
    M: Serialize,
    V: Serialize,
{
    type Ok = ();
    type Error = InnerError;
    type FieldEncoder = ArrayFieldEncoder<&'a mut Serializer<IoWrite<W>>>;

    fn start(self, kind: KnownKind) -> Result<Self::FieldEncoder, MessageError<Self::Error>> {
        msg::MessageEncoder::<M, V>::start(ArrayEncoder::new(&mut self.inner), kind)
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

impl<'a, M, V, R> msg::MessageDecoder<M, V> for &'a mut MessageDecoder<R>
where
    R: Read,
    M: Deserialize<'a>,
    V: Deserialize<'a>,
    V: IntoBasicValue<ConcreteBasicValue<M, V>, M, V>,
    V::Error: Into<MessageError<InnerError>>,
{
    type Error = InnerError;
    type FieldDecoder = ArrayFieldDecoder<M, V, InnerError>;

    fn start(self) -> Result<(KnownKind, Self::FieldDecoder), MessageError<Self::Error>> {
        msg::MessageDecoder::<M, V>::start(ArrayDecoder::new(&mut self.inner))
    }
}

pub struct MessageWriter<W: Write> {
    inner: Serializer<IoWrite<W>>,
}

impl<W> msg::MessageWriter<W> for MessageWriter<W>
where
    W: Write,
{
    type Map = Map;
    type Val = Val;
    type Error = Error;

    fn write_message<M>(&mut self, message: &M) -> Result<(), Self::Error>
    where
        M: Message<Self::Map, Self::Val>,
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
    type Map = Map;
    type Val = Val;
    type Error = Error;

    fn read_message<M>(&mut self) -> Result<M, Self::Error>
    where
        M: Message<Self::Map, Self::Val>,
    {
        M::decode(ArrayDecoder::new(&mut self.inner))
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
    use crate::message::{HelloMessage, Message, StandardMessage};
    use crate::types::{Body, Meta};

    #[test]
    fn test_message_encoder_decoder() {
        let src_message = HelloMessage::new(
            Body::new(Value::Text("1".into())),
            Meta::new(Map::default()),
        );
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
        let message = StandardMessage::<Map, Val>::decode(&mut decoder).unwrap();
        match message {
            StandardMessage::Hello(_) => (),
            other => panic!("unexpected message {:?}", other),
        }
    }
}
