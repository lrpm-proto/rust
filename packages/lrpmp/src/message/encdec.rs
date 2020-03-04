use std::collections::VecDeque;
use std::marker::PhantomData;

use super::MessageError;
use crate::types::{BasicValue, FromBasicValue, FromBasicValuePart, KnownKind};

pub trait MessageEncoder<M, V> {
    type Ok;
    type Error;
    type FieldEncoder: MessageFieldEncoder<M, V, Ok = Self::Ok, Error = Self::Error>;

    fn start(self, kind: KnownKind) -> Result<Self::FieldEncoder, MessageError<Self::Error>>;
}

pub trait MessageFieldEncoder<M, V> {
    type Ok;
    type Error;

    fn encode_field<F>(
        &mut self,
        name: Option<&'static str>,
        value: F,
    ) -> Result<(), MessageError<Self::Error>>
    where
        F: BasicValue<M, V>,
    {
        self.encode_field_ref(name, &value)
    }

    fn encode_field_ref<F>(
        &mut self,
        name: Option<&'static str>,
        value: &F,
    ) -> Result<(), MessageError<Self::Error>>
    where
        F: BasicValue<M, V>;

    fn end(self) -> Result<Self::Ok, MessageError<Self::Error>>;
}

///////////////////////////////////////////////////////////////////////////////

pub trait MessageDecoder<M, V> {
    type Error;
    type FieldDecoder: MessageFieldDecoder<M, V, Error = Self::Error>;

    fn start(self) -> Result<(KnownKind, Self::FieldDecoder), MessageError<Self::Error>>;
}

pub trait MessageFieldDecoder<M, V> {
    type Error;

    fn remaining(&self) -> Option<usize>;

    fn decode_field<T>(
        &mut self,
        name: Option<&'static str>,
    ) -> Result<T, MessageError<Self::Error>>
    where
        T: FromBasicValuePart<M, V>,
        T::Error: Into<MessageError<Self::Error>>;
}

///////////////////////////////////////////////////////////////////////////////

pub struct KindDecoder<D> {
    kind: KnownKind,
    field_decoder: D,
}

impl<D> KindDecoder<D> {
    pub fn new(kind: KnownKind, field_decoder: D) -> Self {
        Self {
            kind,
            field_decoder,
        }
    }
}

impl<M, V, D> MessageDecoder<M, V> for KindDecoder<D>
where
    D: MessageFieldDecoder<M, V>,
{
    type Error = D::Error;
    type FieldDecoder = D;

    fn start(self) -> Result<(KnownKind, Self::FieldDecoder), MessageError<Self::Error>> {
        Ok((self.kind, self.field_decoder))
    }
}

///////////////////////////////////////////////////////////////////////////////

pub struct ArrayFieldDecoder<B, M, V>
where
    B: BasicValue<M, V>,
{
    fields: VecDeque<B>,
    marker: PhantomData<(M, V)>,
}

impl<B, M, V> ArrayFieldDecoder<B, M, V>
where
    B: BasicValue<M, V>,
{
    pub fn new(fields: VecDeque<B>) -> Self {
        Self {
            fields,
            marker: PhantomData,
        }
    }
}

impl<B, M, V> MessageFieldDecoder<M, V> for ArrayFieldDecoder<B, M, V>
where
    B: BasicValue<M, V>,
{
    type Error = ();

    fn remaining(&self) -> Option<usize> {
        Some(self.fields.len())
    }

    fn decode_field<T>(
        &mut self,
        _name: Option<&'static str>,
    ) -> Result<T, MessageError<Self::Error>>
    where
        T: FromBasicValuePart<M, V>,
        T::Error: Into<MessageError<Self::Error>>,
    {
        let value = self.fields.pop_front().ok_or(MessageError::<()>::Eof)?;
        T::from_basic(value).map_err(Into::into)
    }
}
