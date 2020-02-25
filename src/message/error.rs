use super::{BasicType, Kind};

pub enum MessageError<E> {
    Eof,
    Codec(E),
    UnexpectedKind(Kind),
    UnexpectedType(BasicType),
    Custom(&'static str),
}

impl<E> MessageError<E> {
    pub fn for_codec(err: MessageError<()>) -> Self {
        use MessageError::*;
        match err {
            Eof => Eof,
            Codec(()) => Custom("unspecified codec error"),
            UnexpectedKind(k) => UnexpectedKind(k),
            UnexpectedType(b) => UnexpectedType(b),
            Custom(c) => Custom(c),
        }
    }
}
