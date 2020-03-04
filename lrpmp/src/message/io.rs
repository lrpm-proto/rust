use crate::io::{Read, Write};

use super::Message;

pub trait MessageWriter<W>
where
    W: Write,
{
    type Map;
    type Val;
    type Error;

    fn write_message<M>(&mut self, message: &M) -> Result<(), Self::Error>
    where
        M: Message<Self::Map, Self::Val>;
}

pub trait MessageReader<R>
where
    R: Read,
{
    type Map;
    type Val;
    type Error;

    fn read_message<M>(&mut self) -> Result<M, Self::Error>
    where
        M: Message<Self::Map, Self::Val>;
}
