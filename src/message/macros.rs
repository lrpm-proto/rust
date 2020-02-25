macro_rules! impl_standard_kind {
    (
        $(
            (
                $(#[$attr:meta])*
                $name:ident,
                $name_str:expr,
                $code:expr,
                $fields_count:expr
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

            pub fn name(self) -> &'static str {
                match self {
                    $(Self::$name => $name_str),*
                }
            }

            /// Returns the lower and upper bound of the number of fields in the message kind.
            pub fn field_count(&self) -> (usize, Option<usize>) {
                match self {
                    $(Self::$name => ($fields_count + 1, Some($fields_count + 1))),*
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

            fn encode<E>(self, encoder: E) -> Result<E::Ok, MessageError<E::Error>>
            where
                E: MessageEncoder<V>
            {
                match self {
                    $(Self::$kind(m) => m.encode(encoder)),*
                }
            }

            fn encode_ref<E>(&self, encoder: E) -> Result<E::Ok, MessageError<E::Error>>
            where
                E: MessageEncoder<V>
            {
                match self {
                    $(Self::$kind(m) => m.encode_ref(encoder)),*
                }
            }

            fn decode<D>(decoder: D) -> Result<Self, MessageError<D::Error>>
            where
                D: MessageDecoder<V>
            {
                let (kind, decoder) = decoder.start()?;
                let decoder = KindDecoder::new(kind, decoder);
                let std_kind = match kind {
                    k @ KnownKind::Custom(_) => {
                        return Err(MessageError::UnexpectedKind(Kind::Known(k)).into())
                    },
                    KnownKind::Standard(k) => k,
                };

                let message = match std_kind {
                    $(
                        StandardKind::$kind => StandardMessage::$kind($name::decode(decoder)?)
                    ),*
                };

                Ok(message)
            }

            // fn into_standard(self) -> Option<Self> {
            //     Some(self)
            // }
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
        }

        impl<V> $name<V> {
            pub fn new(
                $($field: $field_ty),*,
            ) -> Self {
                Self {
                    $($field),*,
                }
            }
        }

        impl<V> Message<V> for $name<V> {
            fn kind(&self) -> KnownKind {
                KnownKind::Standard(StandardKind::$kind)
            }

            fn encode<E>(self, encoder: E) -> Result<E::Ok, MessageError<E::Error>>
            where
                E: MessageEncoder<V>,
            {
                let mut encoder = encoder.start(self.kind())?;
                $(
                    encoder.encode_field(
                        Some(stringify!($field)),
                        self.$field
                    )?;
                )*
                encoder.end()
            }

            fn encode_ref<E>(&self, encoder: E) -> Result<E::Ok, MessageError<E::Error>>
            where
                E: MessageEncoder<V>,
            {
                let mut encoder = encoder.start(self.kind())?;
                $(
                    encoder.encode_field_ref(
                        Some(stringify!($field)),
                        &self.$field
                    )?;
                )*
                encoder.end()
            }

            fn decode<D>(decoder: D) -> Result<Self, MessageError<D::Error>>
            where
                D: MessageDecoder<V>
            {
                let (kind, mut decoder) = decoder.start()?;
                // TODO: better eq
                if kind != KnownKind::Standard(StandardKind::$kind) {
                    return Err(MessageError::UnexpectedKind(Kind::Known(kind)).into());
                }
                Ok(Self {
                    $(
                        $field: decoder.decode_field::<$field_ty>(Some(stringify!($field)))?
                    ),*,
                })
            }

            // fn into_standard(self) -> Option<StandardMessage<V>> {
            //     Some(StandardMessage::$kind(self))
            // }
        }
    };
}
