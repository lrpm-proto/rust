use std::io::{Read, Write};

use super::Message;

pub trait MessageWriter<W>
where
    W: Write,
{
    type Value;
    type Error;

    fn write_message<M>(&mut self, message: &M) -> Result<usize, Self::Error>
    where
        M: Message<Self::Value>;
}

pub trait MessageReader<R>
where
    R: Read,
{
    type Value;
    type Error;

    fn read_message<M>(&mut self) -> Result<M, Self::Error>
    where
        M: Message<Self::Value>;
}
