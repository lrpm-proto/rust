extern crate proc_macro;

mod imp;

use proc_macro::TokenStream;
use proc_macro_hack::proc_macro_hack;
use quote::quote;
use syn::{parse_macro_input, LitStr};

use lrpmp_spec::uri::UriAnalysis;

use self::imp::impl_std_messages as inner_impl_std_messages;

#[proc_macro]
pub fn impl_std_messages(tokens: TokenStream) -> TokenStream {
    let spec_path = parse_macro_input!(tokens as Option<LitStr>);
    let spec_path = spec_path.map(|lit_str| lit_str.value());

    inner_impl_std_messages(spec_path).into()
}

#[proc_macro_hack]
pub fn uri(tokens: TokenStream) -> TokenStream {
    let uri_lit_str =
        parse_macro_input!(tokens as Option<LitStr>).expect("uri! expects a str literal");
    let uri_str = uri_lit_str.value();

    let uri_parts = UriAnalysis::assert_valid(uri_str.as_str());

    quote!(unsafe {
        ::lrpmp::types::Uri::from_static_parts_unchecked(#uri_lit_str, #uri_parts)
    })
    .into()
}
