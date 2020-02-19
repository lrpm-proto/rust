macro_rules! impl_all_standard_messages {
    (
        $(
            (
                $(#[$struct_attr:meta])*
                $name:ident,
                $kind:ident,
                [ $(
                    $(#[$field_attr:meta])*
                    $field:ident: $field_ty:ty
                ),* ]
            )
        ),*
    ) => {
        /// Enum of all standard messages.
        #[derive(Debug, Clone)]
        pub enum StandardMessage<'a, V> {
            $($kind($name<'a, V>)),*
        }

        impl<'a, V> Message<V> for StandardMessage<'a, V> {
            fn kind(&self) -> KnownKind {
                match self {
                    $(Self::$kind(m) => m.kind()),*
                }
            }

            fn encode<E>(&self, encoder: E) -> Result<(), E::Error>
            where
                E: MessageEncoder<Value = V>
            {
                match self {
                    $(Self::$kind(m) => m.encode(encoder)),*
                }
            }

            fn decode<D>(kind: Kind, decoder: D) -> Result<Self, D::Error>
            where
                D: MessageDecoder<Value = V>
            {
                let std_kind = match kind {
                    k @ Kind::Unknown(_) | k @ Kind::Known(KnownKind::Custom(_)) => {
                        return Err(MessageDecodeError::UnexpectedKind(k).into())
                    },
                    Kind::Known(KnownKind::Standard(k)) => k,
                };

                let message = match std_kind {
                    $(
                        StandardKind::$kind => StandardMessage::$kind($name::decode(kind, decoder)?)
                    ),*
                };

                Ok(message)
            }
        }

        $(
            impl_standard_message!(
                $(#[$struct_attr])*
                $name,
                $kind,
                [
                    $(
                        $(#[$field_attr])*
                        $field: $field_ty
                    ),*
                ]
            );
        )*
    };
}

macro_rules! impl_standard_message {
    (
        $(#[$struct_attr:meta])*
        $name:ident,
        $kind:ident,
        [ $(
            $(#[$field_attr:meta])*
            $field:ident: $field_ty:ty
        ),* ]
    ) => {
        #[derive(Debug, Clone)]
        $(#[$struct_attr])*
        pub struct $name<'a, V> {
            $(
                $(#[$field_attr])*
                pub $field: $field_ty
            ),*,
            #[doc="Optional meta information on this message."]
            pub meta: Meta<V>,
            _marker: PhantomData::<&'a ()>,
        }

        impl<'a, V> $name<'a, V> {
            pub fn new(
                $($field: $field_ty),*,
                meta: Meta<V>,
            ) -> Self {
                Self {
                    $($field),*,
                    meta,
                    _marker: PhantomData,
                }
            }
        }

        impl<'a, V> Message<V> for $name<'a, V> {
            fn kind(&self) -> KnownKind {
                KnownKind::Standard(StandardKind::$kind)
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

            fn decode<D>(_kind: Kind, _decoder: D) -> Result<Self, D::Error>
            where
                D: MessageDecoder<Value = V>
            {
                unimplemented!()
            }
        }
    };
}
