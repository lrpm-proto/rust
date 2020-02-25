use super::{BasicValue, FromBasicValue, KnownKind, Message, MessageError};

pub trait MessageEncoder<V> {
    type Error;
    type FieldEncoder: MessageFieldEncoder<V, Error = Self::Error>;

    fn for_message<M>(self, message: &M) -> Result<Self::FieldEncoder, MessageError<Self::Error>>
    where
        M: Message<V>;
}

pub trait MessageFieldEncoder<V> {
    type Error;

    fn encode_field<T>(
        &mut self,
        name: Option<&'static str>,
        value: T,
    ) -> Result<(), MessageError<Self::Error>>
    where
        T: Into<BasicValue<V>>;
}

///////////////////////////////////////////////////////////////////////////////

pub trait MessageDecoder<V> {
    type Error;
    type FieldDecoder: MessageFieldDecoder<V, Error = Self::Error>;

    fn for_message(self, kind: KnownKind) -> Result<Self::FieldDecoder, MessageError<Self::Error>>;
}

pub trait MessageFieldDecoder<V> {
    type Error;

    fn remaining(&self) -> Option<usize> {
        None
    }

    fn decode_field<T>(
        &mut self,
        name: Option<&'static str>,
    ) -> Result<T, MessageError<Self::Error>>
    where
        T: FromBasicValue<V>;
}

// pub trait MessageTranslation<V> {
//     type Value;
//     type Error;

//     fn translate<I, O>(&self, message: I) -> Result<O, Self::Error>
//     where
//         I: Message<V>,
//         O: Message<Self::Value>;
// }
