use std::collections::VecDeque;
use std::fmt;
use std::marker::PhantomData;

use serde::de::{Deserialize, Deserializer, SeqAccess, Visitor};
use serde::ser::{Serialize, SerializeSeq, Serializer};

use super::{AsBasicValueRef, BasicType, BasicValue, BasicValueRef, FromBasicValue, Kind, Message};

pub trait MessageCodec {
    type Value;
    type Error;
}

pub enum MessageDecodeError<C> {
    Codec(C),
    UnexpectedKind(Kind),
    UnexpectedType(BasicType),
    UnexpectedEof,
}

///////////////////////////////////////////////////////////////////////////////

pub trait MessageEncoder<C: MessageCodec>: Sized {
    type FieldEncoder: MessageFieldEncoder<C>;

    fn for_message<M>(self, message: &M) -> Result<Self::FieldEncoder, C::Error>
    where
        M: Message<C::Value>;
}

pub trait MessageFieldEncoder<C: MessageCodec> {
    fn encode_field<'a, T>(&mut self, name: &'static str, value: &'a T) -> Result<(), C::Error>
    where
        C::Value: 'a,
        T: AsBasicValueRef<'a, C::Value>;
}

///////////////////////////////////////////////////////////////////////////////

pub trait MessageDecoder<'de, C: MessageCodec> {
    type FieldDecoder: MessageFieldDecoder<'de, C::Value>;

    fn for_message(self, kind: &Kind) -> Result<Self::FieldDecoder, MessageDecodeError<C::Error>>;
}

pub trait MessageFieldDecoder<'de, C: MessageCodec> {
    fn decode_field<T>(&mut self, name: &'static str) -> Result<T, MessageDecodeError<C::Error>>
    where
        T: FromBasicValue<C::Value>;
}

///////////////////////////////////////////////////////////////////////////////

pub struct ArrayCodec<C>(pub C);

struct SerdeArrayFieldEncoder<S: Serializer>(S::SerializeSeq);

impl<C> MessageEncoder<C> for ArrayCodec<C>
where
    C: MessageCodec<Error = <C as Serializer>::Error>,
    C: Serializer,
    C::Value: Serialize,
{
    type FieldEncoder = SerdeArrayFieldEncoder<C>;

    fn for_message<M>(self, message: &M) -> Result<Self::FieldEncoder, <C as MessageCodec>::Error>
    where
        M: Message<C::Value>,
    {
        let mut seq = self.0.serialize_seq(message.field_count().1)?;
        seq.serialize_element(&message.kind().code())?;
        Ok(SerdeArrayFieldEncoder(seq))
    }
}

impl<C> MessageFieldEncoder<C> for SerdeArrayFieldEncoder<C>
where
    C: MessageCodec<Error = <C as Serializer>::Error>,
    C: Serializer,
    C::Value: Serialize,
{
    fn encode_field<'a, T>(
        &mut self,
        _name: &'static str,
        value: &'a T,
    ) -> Result<(), <C as MessageCodec>::Error>
    where
        C::Value: 'a,
        T: AsBasicValueRef<'a, C::Value>,
    {
        use BasicValueRef::*;
        match value.as_basic_value_ref() {
            U8(v) => self.0.serialize_element(&v),
            U64(v) => self.0.serialize_element(&v),
            Str(v) => self.0.serialize_element(v),
            Map(v) => self.0.serialize_element(v),
            Val(v) => self.0.serialize_element(v),
        }
    }
}

struct SerdeArrayFieldDecoder<C: MessageCodec>(VecDeque<C::Value>);

impl<'de, C> MessageDecoder<'de, C> for ArrayCodec<C>
where
    C: MessageCodec<Error = <C as Deserializer<'de>>::Error>,
    C: Deserializer<'de>,
    C::Value: Deserialize<'de>,
{
    type FieldDecoder = SerdeArrayFieldDecoder<C>;

    fn for_message(
        self,
        kind: &Kind,
    ) -> Result<Self::FieldDecoder, MessageDecodeError<<C as MessageCodec>::Error>> {
        let values = VecDeque::<C::Value>::deserialize(self.0).map(MessageDecodeError::Codec)?;
        Ok(SerdeArrayFieldDecoder(values))
    }
}

impl<'de, C> MessageFieldDecoder<'de, C> for ArrayCodec<C>
where
    C: MessageCodec<Error = <C as Deserializer<'de>>::Error>,
    C: Deserializer<'de>,
    C::Value: Deserialize<'de>,
{
    fn decode_field<T>(
        &mut self,
        name: &'static str,
    ) -> Result<T, MessageDecodeError<<C as MessageCodec>::Error>>
    where
        T: FromBasicValue<C::Value>,
    {
        unimplemented!()
    }
}

// struct ArrayFieldDecoder<V>(VecDeque<V>);

// impl<V> MessageFieldDecoder<V> for ArrayFieldDecoder<V> {
//     type Error = MessageDecodeError;

//     fn decode_field<T>(&mut self, name: &'static str) -> Result<T, Self::Error>
//     where
//         T: FromBasicValue<V>,
//     {
//         let value = self.0.pop_front();
//         if T::expected_types() == &[BasicType::Val] {
//             return Ok(T::from_basic_value(BasicValue::Val(value)).unwrap());
//         }

//         unimplemented!()
//     }
// }

// // impl<'de, C, V> Visitor<'de> for ArrayFieldDecoder<C, V> {
// //     type Value = Vec<BasicValue<V>>;

// //     fn visit_seq<A>(self, seq: A) -> Result<Self::Value, A::Error>
// //     where
// //         A: SeqAccess<'de>,
// //     {
// //         let values_cap = seq.size_hint().unwrap_or(8);
// //         let mut values = Vec::with_capacity(values_cap);
// //         while let Some(value) = seq.next_element()? {
// //             values.push(value)
// //         }
// //         Ok(values)
// //     }
// // }

// // pub struct BasicValueArray<'de, V> {
// //     values:
// // }

// // impl<'de, V> Visitor<'de> for BasicValueVisitor<V> {
// //     type Value = BasicValue<V>;
// // }

// struct BasicValueVisitor<V> {
//     expecting: &'static [BasicType],
//     value_typ: PhantomData<V>,
// }

// impl<'de, V> Visitor<'de> for BasicValueVisitor<V>
// where
//     V: Visitor<'de>,
// {
//     type Value = BasicValue<V>;

//     fn expecting(&self, f: &mut fmt::Formatter) -> fmt::Result {
//         write!(f, "expecting basic types: {:?}", self.expecting)
//     }
// }
