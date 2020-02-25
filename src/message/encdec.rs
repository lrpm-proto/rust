use super::{AsBasicValueRef, BasicValue, FromBasicValue, KnownKind, MessageError};

pub trait MessageEncoder<V> {
    type Ok;
    type Error;
    type FieldEncoder: MessageFieldEncoder<V, Ok = Self::Ok, Error = Self::Error>;

    fn start(self, kind: KnownKind) -> Result<Self::FieldEncoder, MessageError<Self::Error>>;
}

pub trait MessageFieldEncoder<V> {
    type Ok;
    type Error;

    fn encode_field<F>(
        &mut self,
        name: Option<&'static str>,
        value: F,
    ) -> Result<(), MessageError<Self::Error>>
    where
        F: Into<BasicValue<V>>,
    {
        self.encode_field_ref(name, &value.into())
    }

    fn encode_field_ref<'f, F>(
        &mut self,
        name: Option<&'static str>,
        value: &'f F,
    ) -> Result<(), MessageError<Self::Error>>
    where
        V: 'f,
        F: AsBasicValueRef<'f, V>;

    fn end(self) -> Result<Self::Ok, MessageError<Self::Error>>;
}

///////////////////////////////////////////////////////////////////////////////

pub trait MessageDecoder<V> {
    type Error;
    type FieldDecoder: MessageFieldDecoder<V, Error = Self::Error>;

    fn start(self) -> Result<(KnownKind, Self::FieldDecoder), MessageError<Self::Error>>;
}

pub trait MessageFieldDecoder<V> {
    type Error;

    fn remaining(&self) -> Option<usize>;

    fn decode_field<T>(
        &mut self,
        name: Option<&'static str>,
    ) -> Result<T, MessageError<Self::Error>>
    where
        T: FromBasicValue<V>,
        T::Error: Into<MessageError<Self::Error>>;
}

///////////////////////////////////////////////////////////////////////////////

pub struct KindDecoder<D> {
    kind: KnownKind,
    field_decoder: D,
}

impl<D> KindDecoder<D> {
    pub fn new(kind: KnownKind, field_decoder: D) -> Self {
        Self {
            kind,
            field_decoder,
        }
    }
}

impl<'dec, V, D> MessageDecoder<V> for KindDecoder<D>
where
    D: MessageFieldDecoder<V>,
{
    type Error = D::Error;
    type FieldDecoder = D;

    fn start(self) -> Result<(KnownKind, Self::FieldDecoder), MessageError<Self::Error>> {
        Ok((self.kind, self.field_decoder))
    }
}
