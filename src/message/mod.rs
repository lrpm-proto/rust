pub mod types;

use std::marker::PhantomData;

use self::types::*;

pub trait MessageEncoder {
    type Value;
    type Error;

    fn encode_field<'a, T>(&mut self, name: &'static str, value: &'a T) -> Result<(), Self::Error>
    where
        T: AsBasicTypeRef<'a, Self::Value>;
}

pub trait Message<V> {
    fn kind_str(&self) -> &'static str;

    fn encode<E>(&self, encoder: E) -> Result<(), E::Error>
    where
        E: MessageEncoder<Value = V>;
}

pub struct StandardMessage {}

macro_rules! impl_standard_message {
    (
        $name:ident,
        $kind:ident,
        [ $($field:ident: $field_ty:ty),* ]
    ) => {
        #[derive(Debug, Clone)]
        pub struct $name<'a, V> {
            $(pub $field: $field_ty),*,
            pub meta: Meta<V>,
            _marker: PhantomData::<&'a ()>,
        }

        impl<'a, V> Message<V> for ErrorMessage<'a, V> {
            fn kind_str(&self) -> &'static str {
                StandardKind::$kind.to_str()
            }

            fn encode<E>(&self, mut encoder: E) -> Result<(), E::Error>
            where
                E: MessageEncoder<Value = V>,
            {
                $(
                    encoder.encode_field(
                        stringify!($field),
                        &self.$field
                    )?;
                )*
                Ok(())
            }
        }
    };
}

impl_standard_message!(
    ErrorMessage,
    Error,
    [
        request_kind: Kind,
        request_id: Id,
        error: UriRef<'a>,
        body: Body<V>
    ]
);
