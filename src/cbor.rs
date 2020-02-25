use std::io::{Write, Read};

use serde::Serialize;

pub use serde_cbor::{Value, Error as InnerError, Serializer};

use crate::message::{self as msg, Message, MessageError, StandardMessage};

pub type Error = MessageError<InnerError>;


pub struct MessageTranslation;

impl<V> msg::MessageTranslation<V> for MessageTranslation
where
    V: Serialize,
{
    type Value = Value;
    type Error = Error;

    fn translate<I, O>(message: I) -> Result<O, Self::Error>
    where
        I: Message<V>,
        O: Message<Self::Value>
    {
        SerdeTranslator.translate(message)
    }
}



// pub struct MessageWriter<W: Write> {
//     w: Serializer<W>,
// }

// impl<W> msg::MessageWriter<W> for MessageWriter<W>
// where
//     W: Write,
// {
//     type Value = Value;
//     type Error = Error;

//     fn write_message<M>(&mut self, message: &M) -> Result<usize, Self::Error>
//     where
//         M: Message<Self::Value>,
//     {
//         unimplemented!()
//     }
// }

// pub struct MessageReader<R: Read> {
//     r: R,
// }

// impl<R> msg::MessageReader<R> for MessageReader<R>
// where
//     R: Read,
// {
//     type Value = Value;
//     type Error = Error;

//     fn read_message<M>(&mut self) -> Result<M, Self::Error>
//     where
//         M: Message<Self::Value>,
//     {
//         unimplemented!()
//     }
// }
