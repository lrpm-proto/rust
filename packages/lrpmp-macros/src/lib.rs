extern crate proc_macro;

mod imp;

use proc_macro::TokenStream;
use syn::{parse_macro_input, LitStr};

use self::imp::impl_std_messages as inner_impl_std_messages;

#[proc_macro]
pub fn impl_std_messages(tokens: TokenStream) -> TokenStream {
    let spec_path = parse_macro_input!(tokens as Option<LitStr>);
    let spec_path = spec_path.map(|lit_str| lit_str.value());

    inner_impl_std_messages(spec_path).into()
}
