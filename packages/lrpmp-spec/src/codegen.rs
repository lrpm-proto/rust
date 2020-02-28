use proc_macro2::{Span, TokenStream};
use quote::quote;
use syn::Ident;

use super::{MsgDef, Spec};

fn map_msg_ty<S: AsRef<str>>(ty: S) -> TokenStream {
    let ty = ty.as_ref();
    match ty {
        "Id" => quote!(Id),
        "Uri" => quote!(Uri),
        "Kind" => quote!(Kind),
        "Meta" => quote!(Meta<V>),
        "Body" => quote!(Body<V>),
        _ => panic!("unknown type: {}", ty),
    }
}

fn iden<S: AsRef<str>>(ident: S) -> Ident {
    Ident::new(ident.as_ref(), Span::call_site())
}

pub fn gen_message(def: &MsgDef) -> TokenStream {
    let struct_ident = iden(&def.name);
    let field_idents = def.fields.iter().map(|f| iden(&f.name));
    let field_types = def.fields.iter().map(|f| map_msg_ty(&f.ty));
    quote!(
        pub struct #struct_ident<V> {
            #(#field_idents: #field_types),*
        }
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_msg_gen() {
        let spec = Spec::default().rust_rename();
        for msg in spec.messages {
            gen_message(&msg);
        }
    }
}
