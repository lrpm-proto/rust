use proc_macro2::{Span, TokenStream};
use quote::quote;
use syn::Ident;

use super::{MsgDef, MsgFieldDef, Spec};

fn type_mapping<S: AsRef<str>>(ty: S) -> TokenStream {
    let ty = ty.as_ref();
    match ty {
        "Uri" => quote!(Uri),
        "Meta" => quote!(Meta<V>),
        _ => panic!("unknown type: {}", ty),
    }
}

fn iden<S: AsRef<str>>(ident: S) -> Ident {
    Ident::new(ident.as_ref(), Span::call_site())
}

// def.rust_rename();
pub fn gen_message(def: &MsgDef) -> TokenStream {
    let struct_ident = iden(&def.name);
    let field_idents = def.fields.iter().map(|f| iden(&f.name));
    let field_types = def.fields.iter().map(|f| type_mapping(&f.ty));
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
        let msg = spec.messages.first().unwrap();
        panic!("{}", gen_message(&msg));
    }
}
