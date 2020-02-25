use std::collections::VecDeque;
// use std::fmt;

use std::marker::PhantomData;

use serde::de::{Deserialize, Deserializer};
use serde::ser::{Serialize, SerializeSeq, Serializer};

use crate::message::basic::{
    AsBasicValueRef, BasicType, BasicValue, BasicValueRef, FromBasicValue,
};
use crate::message::special::KnownKind;
use crate::message::{
    Message, MessageDecoder, MessageEncoder, MessageError, MessageFieldDecoder, MessageFieldEncoder,
};

// pub struct SerdeMessageTranslation<V, E> {
//     value: PhantomData<V>,
//     error: PhantomData<E>,
// }

// impl<VI, VO, E> msg::MessageTranslation<VI> for SerdeMessageTranslation<VO, E>
// where
//     VI: Serialize,
// {
//     type Value = VO;
//     type Error = MessageError<E>;

//     fn translate<I, O>(message: I) -> Result<O, Self::Error>
//     where
//         I: Message<VI>,
//         O: Message<Self::Value>
//     {
//         unimplemented!()
//     }
// }

pub struct ArrayEncoder<S>
where
    S: Serializer,
{
    inner: S,
}

impl<V, S> MessageEncoder<V> for ArrayEncoder<S>
where
    S: Serializer,
    V: Serialize,
{
    type Error = S::Error;
    type FieldEncoder = ArrayFieldEncoder<S>;

    fn for_message<M>(self, message: &M) -> Result<Self::FieldEncoder, MessageError<S::Error>>
    where
        M: Message<V>,
    {
        let mut seq = self
            .inner
            .serialize_seq(message.field_count().1)
            .map_err(MessageError::Codec)?;
        seq.serialize_element(&message.kind().code())
            .map_err(MessageError::Codec)?;
        Ok(ArrayFieldEncoder(seq))
    }
}

pub struct ArrayFieldEncoder<S: Serializer>(S::SerializeSeq);

impl<V, S> MessageFieldEncoder<V> for ArrayFieldEncoder<S>
where
    S: Serializer,
    V: Serialize,
{
    type Error = S::Error;

    fn encode_field<T>(
        &mut self,
        _name: Option<&'static str>,
        value: T,
    ) -> Result<(), MessageError<S::Error>>
    where
        T: Into<BasicValue<V>>,
    {
        use BasicValue::*;
        match value.into() {
            U8(v) => self.0.serialize_element(&v),
            U64(v) => self.0.serialize_element(&v),
            Str(v) => self.0.serialize_element(&v as &str),
            Map(v) => self.0.serialize_element(&v),
            Val(v) => self.0.serialize_element(&v),
        }
        .map_err(MessageError::Codec)
    }

    fn encode_field_ref<'a, T>(
        &mut self,
        _name: Option<&'static str>,
        value: &'a T,
    ) -> Result<(), MessageError<S::Error>>
    where
        V: 'a,
        T: AsBasicValueRef<'a, V>,
    {
        use BasicValueRef::*;
        match value.as_basic_value_ref() {
            U8(v) => self.0.serialize_element(&v),
            U64(v) => self.0.serialize_element(&v),
            Str(v) => self.0.serialize_element(v),
            Map(v) => self.0.serialize_element(v),
            Val(v) => self.0.serialize_element(v),
        }
        .map_err(MessageError::Codec)
    }
}

pub struct ArrayDecoder<'de, D>
where
    D: Deserializer<'de>,
{
    inner: D,
    lifetime: PhantomData<&'de D>,
}

impl<'de, V, D> MessageDecoder<V> for ArrayDecoder<'de, D>
where
    D: Deserializer<'de>,
    V: Deserialize<'de> + Into<BasicValue<V>>,
{
    type Error = D::Error;
    type FieldDecoder = ArrayFieldDecoder<V, D::Error>;

    fn for_message(self, _kind: KnownKind) -> Result<Self::FieldDecoder, MessageError<D::Error>> {
        match VecDeque::<V>::deserialize(self.inner) {
            Ok(values) => Ok(ArrayFieldDecoder {
                values,
                error: PhantomData,
            }),
            Err(err) => Err(MessageError::Codec(err)),
        }
    }
}

pub struct ArrayFieldDecoder<V, E> {
    values: VecDeque<V>,
    error: PhantomData<E>,
}

impl<V, E> MessageFieldDecoder<V> for ArrayFieldDecoder<V, E>
where
    V: Into<BasicValue<V>>,
{
    type Error = E;

    fn decode_field<T>(&mut self, _name: Option<&'static str>) -> Result<T, MessageError<E>>
    where
        T: FromBasicValue<V>,
        T::Error: Into<MessageError<Self::Error>>,
    {
        let value = self.values.pop_front().ok_or(MessageError::<E>::Eof)?;
        let basic_value = if T::expected_types() == [BasicType::Val] {
            BasicValue::Val(value)
        } else {
            value.into()
        };
        T::from_basic_value(basic_value).map_err(Into::into)
    }
}
