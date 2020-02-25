// use std::io::{Write, Read};

pub trait Write {
    //type Error;

    //fn write_all(&mut self, buf: &[u8]) -> Result<(), Self::Error>;
}

pub trait Read {
    // fn read<'a>(&'a mut self, n: usize) -> Result<EitherLifetime<'a, 'de>> {
    //     self.clear_buffer();
    //     self.read_to_buffer(n)?;
    //     Ok(self.take_buffer())
    // }
}
