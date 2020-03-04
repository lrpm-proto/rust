pub const SEGMENT: u8 = b'.';
pub const WILDCARD: u8 = b'*';

pub enum UriAnalysis {
    Valid(UriParts),
    Invalid { invalid: char, offset: usize },
}

#[derive(Debug, Clone, PartialEq)]
pub struct UriParts {
    pub segment_count: u8,
    pub wildcard_count: u8,
}

#[cfg(feature = "codegen")]
mod uri_macro {
    use super::UriParts;
    use proc_macro2::TokenStream;
    use quote::{quote, ToTokens};

    impl ToTokens for UriParts {
        fn to_tokens(&self, tokens: &mut TokenStream) {
            let Self {
                wildcard_count,
                segment_count,
            } = self;
            tokens.extend(quote!(
                ::lrpmp_spec::uri::UriParts {
                    wildcard_count: #wildcard_count,
                    segment_count: #segment_count,
                },
            ))
        }
    }
}

impl UriAnalysis {
    #[inline]
    pub fn for_uri_bytes(uri: &[u8]) -> Self {
        let mut prev_char = 0;
        let mut segment_count = 0;
        let mut wildcard_count = 0;
        for (i, c) in uri.iter().copied().enumerate() {
            match c {
                WILDCARD => {
                    if prev_char == WILDCARD || wildcard_count == u8::max_value() {
                        return Self::invalid(c, i);
                    }
                    wildcard_count += 1;
                }
                SEGMENT => {
                    if prev_char == SEGMENT || segment_count == u8::max_value() {
                        return Self::invalid(c, i);
                    }
                    segment_count += 1;
                }
                b'_' | b'a'..=b'z' | b'0'..=b'9' => (),
                _ => return Self::invalid(c, i),
            }
            prev_char = c;
        }
        Self::Valid(UriParts {
            segment_count,
            wildcard_count,
        })
    }

    #[inline]
    pub fn assert_valid(uri: &str) -> UriParts {
        match Self::for_uri_bytes(uri.as_bytes()) {
            Self::Invalid { invalid, offset } => {
                panic!(
                    "invalid uri `{}` at char `{}` (offset {})",
                    uri, invalid, offset
                );
            }
            Self::Valid(parts) => parts,
        }
    }

    #[inline]
    fn invalid(invalid: u8, offset: usize) -> Self {
        Self::Invalid {
            invalid: invalid as char,
            offset,
        }
    }
}
