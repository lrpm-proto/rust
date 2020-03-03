use super::*;
use crate::types::{ConcreteBasicValue, KnownKind};

#[derive(Debug, Clone)]
pub struct GenericMessage<M, V> {
    kind: KnownKind,
    fields: Vec<ConcreteBasicValue<M, V>>,
}

impl<M, V> GenericMessage<M, V> {
    pub fn new(kind: KnownKind, fields: Vec<ConcreteBasicValue<M, V>>) -> Self {
        Self { kind, fields }
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
        Ok(Self::new(kind, fields))
    }

    // fn into_standard(self) -> Result<StandardMessage<V>, MessageError<()>> {
    //     unimplemented!()
    // }
}
