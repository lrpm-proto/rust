mod error;
mod io;
mod transmute;

pub mod dec;
pub mod enc;
pub mod generic;

pub use self::dec::MessageDecoder;
pub use self::enc::MessageEncoder;
pub use self::error::*;
pub use self::generic::GenericMessage;
pub use self::io::*;

pub use crate::std_msgs::*;

use self::transmute::*;

use crate::types::KnownKind;

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
    fn into_generic(self) -> GenericMessage<M, V> {
        self.transmute().unwrap()
    }
}

pub trait MessageExt<M, V>: Message<M, V> {
    #[inline]
    fn is_standard(&self) -> bool {
        self.kind().is_standard()
    }

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
