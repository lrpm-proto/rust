use std::slice;

use super::dec::*;
use super::enc::*;
use super::*;
use crate::types::{BasicValue, BasicValueExt, ConcreteBasicValue, KnownKind};

#[derive(Debug, Clone)]
pub struct GenericMessage<M, V> {
    kind: KnownKind,
    fields: Vec<ConcreteBasicValue<M, V>>,
}

impl<M, V> GenericMessage<M, V> {
    pub fn new<B>(kind: KnownKind, fields: Vec<B>) -> Self
    where
        B: BasicValue<M, V>,
    {
        let fields = fields
            .into_iter()
            .map(BasicValueExt::into_concrete)
            .collect();
        Self { kind, fields }
    }

    pub fn field_iter(&self) -> FieldIter<'_, M, V> {
        FieldIter {
            inner: self.fields.iter(),
        }
    }
}

impl<M, V> Message<M, V> for GenericMessage<M, V> {
    fn kind(&self) -> KnownKind {
        self.kind
    }

    fn encode<E>(self, encoder: E) -> Result<E::Ok, MessageError<E::Error>>
    where
        E: MessageEncoder<M, V>,
    {
        let mut encoder = encoder.start(self.kind())?;
        for field in self.fields.into_iter() {
            encoder.encode_field(None, field)?;
        }
        encoder.end()
    }

    fn encode_ref<E>(&self, encoder: E) -> Result<E::Ok, MessageError<E::Error>>
    where
        E: MessageEncoder<M, V>,
    {
        let mut encoder = encoder.start(self.kind())?;
        for field in self.fields.iter() {
            encoder.encode_field_ref(None, field)?;
        }
        encoder.end()
    }

    fn decode<D>(decoder: D) -> Result<Self, MessageError<D::Error>>
    where
        D: MessageDecoder<M, V>,
    {
        let (kind, mut decoder) = decoder.start()?;
        let cap = decoder.remaining().unwrap_or(0);
        let mut fields = Vec::with_capacity(cap);
        while Some(0) != decoder.remaining() {
            fields.push(decoder.decode_field(None)?);
        }
        Ok(Self { kind, fields })
    }
}

pub struct FieldIter<'a, M, V> {
    inner: slice::Iter<'a, ConcreteBasicValue<M, V>>,
}

impl<'a, M, V> Iterator for FieldIter<'a, M, V> {
    type Item = &'a dyn BasicValue<M, V>;

    fn size_hint(&self) -> (usize, Option<usize>) {
        self.inner.size_hint()
    }

    fn next(&mut self) -> Option<Self::Item> {
        self.inner.next().map(|f| f as &_)
    }
}

impl<'a, M, V> ExactSizeIterator for FieldIter<'a, M, V> {}
