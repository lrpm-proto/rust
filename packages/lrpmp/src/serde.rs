use std::collections::VecDeque;
use std::marker::PhantomData;

use serde::de::{Deserialize, Deserializer};
use serde::ser::{Serialize, SerializeSeq, Serializer};

use crate::message::*;
use crate::types::*;

pub struct ArrayEncoder<S> {
    inner: S,
}

impl<S> ArrayEncoder<S>
where
    S: Serializer,
{
    pub fn new(ser: S) -> Self {
        Self { inner: ser }
    }
}

impl<M, V, S> MessageEncoder<M, V> for ArrayEncoder<S>
where
    S: Serializer,
    V: Serialize,
    M: Serialize,
{
    type Ok = S::Ok;
    type Error = S::Error;
    type FieldEncoder = ArrayFieldEncoder<S>;

    fn start(self, kind: KnownKind) -> Result<Self::FieldEncoder, MessageError<S::Error>> {
        let mut seq = self
            .inner
            .serialize_seq(kind.field_count().1.map(|c| {
                c + 1 // account for kind field
            }))
            .map_err(MessageError::Codec)?;
        seq.serialize_element(&kind.code())
            .map_err(MessageError::Codec)?;
        Ok(ArrayFieldEncoder(seq))
    }
}

pub struct ArrayFieldEncoder<S: Serializer>(S::SerializeSeq);

impl<M, V, S> MessageFieldEncoder<M, V> for ArrayFieldEncoder<S>
where
    S: Serializer,
    M: Serialize,
    V: Serialize,
{
    type Ok = S::Ok;
    type Error = S::Error;

    fn encode_field_ref<F>(
        &mut self,
        _name: Option<&'static str>,
        value: &F,
    ) -> Result<(), MessageError<Self::Error>>
    where
        F: BasicValue<M, V>,
    {
        use BasicType::*;
        match value.ty() {
            U8 => self.0.serialize_element(&value.as_u8()),
            U64 => self.0.serialize_element(&value.as_u64()),
            Str => self.0.serialize_element(value.as_str()),
            Map => self.0.serialize_element(value.as_map()),
            Val => self.0.serialize_element(value.as_val()),
        }
        .map_err(MessageError::Codec)
    }

    fn end(self) -> Result<S::Ok, MessageError<S::Error>> {
        self.0.end().map_err(MessageError::Codec)
    }
}

impl<'de, D> ArrayDecoder<'de, D>
where
    D: Deserializer<'de>,
{
    pub fn new(de: D) -> Self {
        Self {
            inner: de,
            lifetime: PhantomData,
        }
    }
}

pub struct ArrayDecoder<'de, D>
where
    D: Deserializer<'de>,
{
    inner: D,
    lifetime: PhantomData<&'de D>,
}

impl<'de, M, V, D> MessageDecoder<M, V> for ArrayDecoder<'de, D>
where
    D: Deserializer<'de>,
    V: Deserialize<'de>,
    V: IntoBasicValue<ConcreteBasicValue<M, V>, M, V>,
    V::Error: Into<MessageError<D::Error>>,
    M: Deserialize<'de>,
{
    type Error = D::Error;
    type FieldDecoder = ArrayFieldDecoder<M, V, D::Error>;

    fn start(self) -> Result<(KnownKind, Self::FieldDecoder), MessageError<D::Error>> {
        match VecDeque::<V>::deserialize(self.inner) {
            Ok(values) => {
                let mut field_decoder = ArrayFieldDecoder {
                    values,
                    marker: PhantomData,
                };
                let kind = field_decoder.decode_field(Some("kind"))?;
                Ok((kind, field_decoder))
            }
            Err(err) => Err(MessageError::Codec(err)),
        }
    }
}

pub struct ArrayFieldDecoder<M, V, E> {
    values: VecDeque<V>,
    marker: PhantomData<(M, E)>,
}

impl<M, V, E> MessageFieldDecoder<M, V> for ArrayFieldDecoder<M, V, E>
where
    V: IntoBasicValue<ConcreteBasicValue<M, V>, M, V>,
    V::Error: Into<MessageError<E>>,
{
    type Error = E;

    fn remaining(&self) -> Option<usize> {
        Some(self.values.len())
    }

    fn decode_field<T>(&mut self, _name: Option<&'static str>) -> Result<T, MessageError<E>>
    where
        T: FromBasicValuePart<M, V>,
        T::Error: Into<MessageError<Self::Error>>,
    {
        let value = self.values.pop_front().ok_or(MessageError::<E>::Eof)?;
        if T::expected_types() == [BasicType::Val] {
            T::from_basic_val(value).map_err(Into::into)
        } else {
            let concrete = value.into_basic().map_err(Into::into)?;
            T::from_basic(concrete).map_err(Into::into)
        }
    }
}
