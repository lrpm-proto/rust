use crate::codec::generic::Map;
use crate::message::{GenericMessage, Message, MessageDecoder, MessageEncoder, MessageError};
use crate::types::KnownKind;

pub struct BusMessage<V> {
    inner: GenericMessage<Map<V>, V>,
}

impl<V> Message<Map<V>, V> for BusMessage<V> {
    fn kind(&self) -> KnownKind {
        self.inner.kind()
    }

    fn encode<E>(self, encoder: E) -> Result<E::Ok, MessageError<E::Error>>
    where
        E: MessageEncoder<Map<V>, V>,
    {
        self.inner.encode(encoder)
    }

    fn encode_ref<E>(&self, encoder: E) -> Result<E::Ok, MessageError<E::Error>>
    where
        E: MessageEncoder<Map<V>, V>,
    {
        self.inner.encode_ref(encoder)
    }

    fn decode<D>(decoder: D) -> Result<Self, MessageError<D::Error>>
    where
        D: MessageDecoder<Map<V>, V>,
    {
        Ok(Self {
            inner: GenericMessage::decode(decoder)?,
        })
    }

    fn into_generic(self) -> GenericMessage<Map<V>, V> {
        self.inner.into_generic()
    }
}
