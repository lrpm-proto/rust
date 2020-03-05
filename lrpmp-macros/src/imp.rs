use proc_macro2::{Span, TokenStream};
use quote::quote;
use syn::{Error, Ident, LitStr};

use lrpmp_spec::uri;
use lrpmp_spec::{MsgDef, Spec};

use crate::spec::get_spec;

fn with_spec<F>(spec_path_opt: Option<String>, f: F) -> TokenStream
where
    F: FnOnce(Spec) -> TokenStream,
{
    match get_spec(spec_path_opt) {
        Ok(spec) => f(spec),
        Err(spec_err) => Error::new(Span::call_site(), spec_err).to_compile_error(),
    }
}

pub fn impl_std_kind(spec_path_opt: Option<String>) -> TokenStream {
    with_spec(spec_path_opt, gen_std_kind)
}

pub fn impl_std_messages(spec_path_opt: Option<String>) -> TokenStream {
    with_spec(spec_path_opt, |spec| {
        let mut out = TokenStream::new();

        for msg in spec.message_iter() {
            out.extend(gen_message(msg));
        }

        out.extend(gen_std_message(spec));
        out
    })
}

pub fn impl_std_uris(spec_path_opt: Option<String>) -> TokenStream {
    with_spec(spec_path_opt, |spec| {
        let mut out = TokenStream::new();

        for uri_def in spec.uri_iter() {
            let name = ident(format!("{}_URI", uri_def.name()));
            let uri_str_lit = LitStr::new(uri_def.uri(), Span::call_site());
            let uri_expr = impl_uri(uri_str_lit);

            out.extend(quote!(
                pub static #name: Uri = #uri_expr;
            ));
        }

        out
    })
}

pub fn impl_uri(uri_lit_str: LitStr) -> TokenStream {
    let uri_str = uri_lit_str.value();
    match uri::validate_bytes(uri_str.as_bytes()) {
        Ok(uri_parts) => quote!(unsafe { Uri::from_static_parts_unchecked(#uri_str, #uri_parts) }),
        Err(err) => Error::new_spanned(uri_lit_str, err.message_with_uri(uri_str.as_ref()))
            .to_compile_error(),
    }
}

fn gen_std_message(spec: Spec) -> TokenStream {
    let kind_idents: Vec<_> = spec.message_iter().map(msg_kind_ident).collect();
    let message_idents: Vec<_> = spec.message_iter().map(msg_struct_ident).collect();

    quote!(
        /// Enum of all standard messages.
        #[derive(Debug, Clone)]
        pub enum StandardMessage<M, V> {
            #(#kind_idents(#message_idents<M, V>)),*
        }

        impl<M, V> Message<M, V> for StandardMessage<M, V> {
            fn kind(&self) -> KnownKind {
                match self {
                    #(Self::#kind_idents(m) => m.kind()),*
                }
            }

            fn encode<E>(self, encoder: E) -> Result<E::Ok, MessageError<E::Error>>
            where
                E: MessageEncoder<M, V>
            {
                match self {
                    #(Self::#kind_idents(m) => m.encode(encoder)),*
                }
            }

            fn encode_ref<E>(&self, encoder: E) -> Result<E::Ok, MessageError<E::Error>>
            where
                E: MessageEncoder<M, V>
            {
                match self {
                    #(Self::#kind_idents(m) => m.encode_ref(encoder)),*
                }
            }

            fn decode<D>(decoder: D) -> Result<Self, MessageError<D::Error>>
            where
                D: MessageDecoder<M, V>
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
                    #(
                        StandardKind::#kind_idents => StandardMessage::#kind_idents(#message_idents::decode(decoder)?)
                    ),*
                };

                Ok(message)
            }

            fn into_standard(self) -> Result<Self, MessageError<()>> {
                Ok(self)
            }
        }
    )
}

fn gen_std_kind(spec: Spec) -> TokenStream {
    let kind_field_counts: Vec<_> = spec.message_iter().map(|m| m.field_iter().len()).collect();
    let kind_names: Vec<_> = spec.message_iter().map(|m| m.kind_name()).collect();
    let kind_idents: Vec<_> = spec.message_iter().map(msg_kind_ident).collect();
    let kind_codes: Vec<_> = spec.message_iter().map(|m| m.kind_code()).collect();

    quote!(
        /// Standard defined message kinds.
        #[derive(Debug, Clone, Copy, PartialEq)]
        #[repr(u8)]
        pub enum StandardKind {
            #(
                #kind_idents = #kind_codes
            ),*
        }

        impl StandardKind {
            pub fn from_name(name: &str) -> Option<Self> {
                match name {
                    #(#kind_names => Some(Self::#kind_idents)),*,
                    _ => None,
                }
            }

            pub fn from_code(code: u8) -> Option<Self> {
                match code {
                    #(#kind_codes => Some(Self::#kind_idents)),*,
                    _ => None,
                }
            }

            pub fn name(self) -> &'static str {
                match self {
                    #(Self::#kind_idents => #kind_names),*
                }
            }

            /// Returns the lower and upper bound of the number of fields in the message kind.
            pub fn field_count(&self) -> (usize, Option<usize>) {
                match self {
                    #(Self::#kind_idents => (#kind_field_counts, Some(#kind_field_counts))),*
                }
            }
        }
    )
}

fn gen_message(def: &MsgDef) -> TokenStream {
    let kind_ident = msg_kind_ident(&def);
    let struct_ident = msg_struct_ident(&def);
    let struct_doc = def.desc();
    let field_names_and_types = msg_field_names_and_types(&def);
    let (field_idents, field_types): (Vec<_>, Vec<_>) = field_names_and_types.into_iter().unzip();

    quote!(
        #[derive(Debug, Clone)]
        #[doc = #struct_doc]
        pub struct #struct_ident<M, V> {
            #(pub #field_idents: #field_types),*,
            // Only allow construction via public methods.
            _seal: (),
        }

        impl<M, V> #struct_ident<M, V> {
            pub fn new(
                #(#field_idents: #field_types),*,
            ) -> Self {
                Self {
                    #(#field_idents),*,
                    _seal: (),
                }
            }
        }

        impl<M, V> Message<M, V> for #struct_ident<M, V> {
            fn kind(&self) -> KnownKind {
                KnownKind::Standard(StandardKind::#kind_ident)
            }

            fn encode<E>(self, encoder: E) -> Result<E::Ok, MessageError<E::Error>>
            where
                E: MessageEncoder<M, V>,
            {
                let mut encoder = encoder.start(self.kind())?;
                #(
                    encoder.encode_field(
                        Some(stringify!(#field_idents)),
                        self.#field_idents
                    )?;
                )*
                encoder.end()
            }

            fn encode_ref<E>(&self, encoder: E) -> Result<E::Ok, MessageError<E::Error>>
            where
                E: MessageEncoder<M, V>,
            {
                let mut encoder = encoder.start(self.kind())?;
                #(
                    encoder.encode_field_ref(
                        Some(stringify!(#field_idents)),
                        &self.#field_idents
                    )?;
                )*
                encoder.end()
            }

            fn decode<D>(decoder: D) -> Result<Self, MessageError<D::Error>>
            where
                D: MessageDecoder<M, V>
            {
                let (kind, mut decoder) = decoder.start()?;
                if kind != KnownKind::Standard(StandardKind::#kind_ident) {
                    return Err(MessageError::UnexpectedKind(Kind::Known(kind)).into());
                }
                Ok(Self {
                    #(
                        #field_idents: decoder.decode_field::<#field_types>(Some(stringify!(#field_idents)))?
                    ),*,
                    _seal: (),
                })
            }

            fn into_standard(self) -> Result<StandardMessage<M, V>, MessageError<()>> {
                Ok(StandardMessage::#kind_ident(self))
            }
        }
    )
}

fn msg_kind_ident(def: &MsgDef) -> Ident {
    ident(def.name())
}

fn msg_struct_ident(def: &MsgDef) -> Ident {
    ident(format!("{}Message", def.name()))
}

fn msg_field_names_and_types(def: &MsgDef) -> Vec<(Ident, TokenStream)> {
    def.field_iter()
        .map(|f| (ident(f.name()), map_msg_ty(f.ty())))
        .collect()
}

fn map_msg_ty<S: AsRef<str>>(ty: S) -> TokenStream {
    let ty = ty.as_ref();
    match ty {
        "Id" => quote!(Id),
        "Uri" => quote!(Uri),
        "Kind" => quote!(Kind),
        "Meta" => quote!(Meta<M, V>),
        "Body" => quote!(Body<V>),
        _ => panic!("unknown type: {}", ty),
    }
}

pub fn ident<S>(ident: S) -> Ident
where
    S: AsRef<str>,
{
    Ident::new(ident.as_ref(), Span::call_site())
}
