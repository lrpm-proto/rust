macro_rules! impl_standard_kind {
    (
        $(
            (
                $(#[$attr:meta])*
                $name:ident,
                $name_str:expr,
                $code:expr
            )
        ),*
    ) => {
        /// Standard defined message kinds.
        #[derive(Debug, Clone, Copy, PartialEq)]
        #[repr(u8)]
        pub enum StandardKind {
            $(
                $(#[$attr:meta])*
                $name = $code
            ),*
        }

        impl StandardKind {
            pub fn from_name(name: &str) -> Option<Self> {
                match name {
                    $($name_str => Some(Self::$name)),*,
                    _ => None,
                }
            }

            pub fn from_code(code: u8) -> Option<Self> {
                match code {
                    $($code => Some(Self::$name)),*,
                    _ => None,
                }
            }

            pub fn to_str(self) -> &'static str {
                match self {
                    $(Self::$name => $name_str),*
                }
            }
        }
    };
}

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
        pub enum StandardMessage<V> {
            $($kind($name<V>)),*
        }

        impl<V> Message<V> for StandardMessage<V> {
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
        pub struct $name<V> {
            $(
                $(#[$field_attr])*
                pub $field: $field_ty
            ),*,
            #[doc="Optional meta information on this message."]
            pub meta: Meta<V>,
        }

        impl<V> $name<V> {
            pub fn new(
                $($field: $field_ty),*,
                meta: Meta<V>,
            ) -> Self {
                Self {
                    $($field),*,
                    meta,
                }
            }
        }

        impl<V> Message<V> for $name<V> {
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

            fn decode<D>(kind: Kind, mut decoder: D) -> Result<Self, D::Error>
            where
                D: MessageDecoder<Value = V>
            {
                // TODO: better eq
                if kind != Kind::Known(KnownKind::Standard(StandardKind::$kind)) {
                    // TODO: error!
                    unimplemented!()
                }
                Ok(Self {
                    $($field: decoder.decode_field::<$field_ty>(stringify!($field))?),*,
                    meta: decoder.decode_field::<Meta<V>>("meta")?
                })
            }
        }
    };
}
