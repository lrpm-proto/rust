pub use serde_cbor::{Deserializer, Error as InnerError, Serializer, Value};

use crate::io::{Read, Write};
use crate::message::{self as msg, Message, MessageError};
use crate::serde::{ArrayDecoder, ArrayEncoder};

pub type Error = MessageError<InnerError>;

pub type MessageEncoder<W> = ArrayEncoder<Serializer<W>>;
pub type MessageDecoder<'de, R> = ArrayDecoder<'de, Deserializer<R>>;

pub struct MessageWriter<W: Write> {
    inner: Serializer<W>,
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
    inner: Deserializer<R>,
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
        // ArrayDecoder::new(&mut self.inner).for_message(None)
        // M::decode(, decoder: D)
        unimplemented!()
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