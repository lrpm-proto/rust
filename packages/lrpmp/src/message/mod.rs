mod encdec;
mod error;
mod generic;
mod io;
mod transmute;

pub use self::encdec::*;
pub use self::error::*;
pub use self::generic::*;
pub use self::io::*;

use self::transmute::*;

use crate::types::KnownKind;

pub use crate::std_impl::*;

pub trait Message<M, V>: Sized {
    /// Returns the message kind.
    fn kind(&self) -> KnownKind;

    /// Consumes and encodes the message given an encoder.
    fn encode<E>(self, encoder: E) -> Result<E::Ok, MessageError<E::Error>>
    where
        E: MessageEncoder<M, V>,
    {
        self.encode_ref(encoder)
    }

    /// Encodes the message given an encoder.
    fn encode_ref<E>(&self, encoder: E) -> Result<E::Ok, MessageError<E::Error>>
    where
        E: MessageEncoder<M, V>;

    /// Decodes the message given a decoder.
    fn decode<D>(decoder: D) -> Result<Self, MessageError<D::Error>>
    where
        D: MessageDecoder<M, V>;

    /// Convert the message into a standard message if applicable.
    fn into_standard(self) -> Result<StandardMessage<M, V>, MessageError<()>> {
        self.transmute()
    }

    /// Convert the message into a generic message.
    fn into_generic<MO, VO>(self) -> GenericMessage<MO, VO>
    where
        MO: From<M>,
        VO: From<V>,
    {
        self.transmute().unwrap()
    }
}

pub trait MessageExt<M, V>: Message<M, V> {
    /// Transmute one message type to another.
    fn transmute<MsgOut, MapOut, ValOut>(self) -> Result<MsgOut, MessageError<()>>
    where
        MsgOut: Message<MapOut, ValOut>,
        MapOut: From<M>,
        ValOut: From<V>,
    {
        self.encode(TransmuteEncoder::new())
    }
}

impl<T, M, V> MessageExt<M, V> for T where T: Message<M, V> {}
