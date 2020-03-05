extern crate proc_macro;

mod imp;
mod spec;

use proc_macro::TokenStream;
use proc_macro_hack::proc_macro_hack;
use syn::{parse_macro_input, LitStr};

use self::imp::impl_std_kind as inner_impl_std_kind;
use self::imp::impl_std_messages as inner_impl_std_messages;
use self::imp::impl_std_uris as inner_impl_std_uris;
use self::imp::impl_uri as inner_impl_uri;

#[proc_macro]
pub fn impl_std_kind(tokens: TokenStream) -> TokenStream {
    let spec_path = parse_macro_input!(tokens as Option<LitStr>);
    let spec_path = spec_path.map(|lit_str| lit_str.value());

    inner_impl_std_kind(spec_path).into()
}

#[proc_macro]
pub fn impl_std_messages(tokens: TokenStream) -> TokenStream {
    let spec_path = parse_macro_input!(tokens as Option<LitStr>);
    let spec_path = spec_path.map(|lit_str| lit_str.value());

    inner_impl_std_messages(spec_path).into()
}

#[proc_macro]
pub fn impl_std_uris(tokens: TokenStream) -> TokenStream {
    let spec_path = parse_macro_input!(tokens as Option<LitStr>);
    let spec_path = spec_path.map(|lit_str| lit_str.value());

    inner_impl_std_uris(spec_path).into()
}

#[proc_macro_hack]
pub fn uri(tokens: TokenStream) -> TokenStream {
    let uri_str_lit = parse_macro_input!(tokens as LitStr);

    inner_impl_uri(uri_str_lit).into()
}
