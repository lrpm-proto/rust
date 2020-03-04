use super::MessageError;
use crate::types::{BasicValue, KnownKind};

pub trait MessageEncoder<M, V> {
    type Ok;
    type Error;
    type FieldEncoder: MessageFieldEncoder<M, V, Ok = Self::Ok, Error = Self::Error>;

    fn start(self, kind: KnownKind) -> Result<Self::FieldEncoder, MessageError<Self::Error>>;
}

pub trait MessageFieldEncoder<M, V> {
    type Ok;
    type Error;

    fn encode_field<F>(
        &mut self,
        name: Option<&'static str>,
        value: F,
    ) -> Result<(), MessageError<Self::Error>>
    where
        F: BasicValue<M, V>,
    {
        self.encode_field_ref(name, &value)
    }

    fn encode_field_ref<F>(
        &mut self,
        name: Option<&'static str>,
        value: &F,
    ) -> Result<(), MessageError<Self::Error>>
    where
        F: BasicValue<M, V>;

    fn end(self) -> Result<Self::Ok, MessageError<Self::Error>>;
}
