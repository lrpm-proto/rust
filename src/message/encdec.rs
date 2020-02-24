use std::convert::TryFrom;

use super::{
    Kind,
    BasicValue,
    AsBasicValueRef,
};

pub enum MessageDecodeError {
    UnexpectedKind(Kind),
}

pub trait MessageEncoder {
    type Value;
    type Error;

    fn encode_field<'a, T>(&mut self, name: &'static str, value: &'a T) -> Result<(), Self::Error>
    where
        T: AsBasicValueRef<'a, Self::Value>;
}

pub trait MessageDecoder {
    type Value;
    type Error: From<MessageDecodeError>;

    fn decode_field<T>(&mut self, name: &'static str) -> Result<T, Self::Error>
    where
        T: TryFrom<BasicValue<Self::Value>>;
}