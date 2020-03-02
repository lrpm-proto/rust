mod encdec;
mod error;
mod io;
mod standard;

pub mod basic;
pub mod special;

use self::basic::*;
use self::special::*;

pub use self::encdec::*;
pub use self::error::*;
pub use self::io::*;
pub use self::standard::*;

pub trait Message<V>: Sized {
    /// Returns the message kind.
    fn kind(&self) -> KnownKind;

    /// Consumes and encodes the message given an encoder.
    fn encode<E>(self, encoder: E) -> Result<E::Ok, MessageError<E::Error>>
    where
        E: MessageEncoder<V>,
    {
        self.encode_ref(encoder)
    }

    /// Encodes the message given an encoder.
    fn encode_ref<E>(&self, encoder: E) -> Result<E::Ok, MessageError<E::Error>>
    where
        E: MessageEncoder<V>;

    /// Decodes the message given basic values and a known kind.
    fn decode<D>(decoder: D) -> Result<Self, MessageError<D::Error>>
    where
        D: MessageDecoder<V>;

    // /// Convert the message into a standard message if applicable.
    // fn into_standard(self) -> Result<StandardMessage<V>, MessageError<()>>;
}

#[derive(Debug, Clone)]
pub struct GenericMessage<V> {
    kind: KnownKind,
    fields: Vec<BasicValue<V>>,
}

impl<V> GenericMessage<V> {
    pub fn new(kind: KnownKind, fields: Vec<BasicValue<V>>) -> Self {
        Self { kind, fields }
    }
}

impl<V> Message<V> for GenericMessage<V> {
    fn kind(&self) -> KnownKind {
        self.kind
    }

    fn encode<E>(self, encoder: E) -> Result<E::Ok, MessageError<E::Error>>
    where
        E: MessageEncoder<V>,
    {
        let mut encoder = encoder.start(self.kind())?;
        for field in self.fields.into_iter() {
            encoder.encode_field(None, field)?;
        }
        encoder.end()
    }

    fn encode_ref<E>(&self, encoder: E) -> Result<E::Ok, MessageError<E::Error>>
    where
        E: MessageEncoder<V>,
    {
        let mut encoder = encoder.start(self.kind())?;
        for field in self.fields.iter() {
            encoder.encode_field_ref(None, field)?;
        }
        encoder.end()
    }

    fn decode<D>(decoder: D) -> Result<Self, MessageError<D::Error>>
    where
        D: MessageDecoder<V>,
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
