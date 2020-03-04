mod encdec;
mod error;
mod generic;
//mod io;

pub use self::encdec::*;
pub use self::error::*;
pub use self::generic::*;
//pub use self::io::*;

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
    fn into_standard(self) -> Result<StandardMessage<M, V>, MessageError<()>>;
}
