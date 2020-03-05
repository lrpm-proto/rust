use crate::{default_naming, Deserialize, NamingConvention};

pub const SEGMENT: u8 = b'.';
pub const WILDCARD: u8 = b'*';

#[derive(Debug, Clone, Deserialize)]
struct UriDefInner {
    uri: String,
    desc: String,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(from = "UriDefInner")]
pub struct UriDef {
    inner: UriDefInner,
    name: String,
    #[serde(default = "default_naming", skip)]
    naming: &'static NamingConvention,
}

impl UriDef {
    pub fn name(&self) -> &str {
        self.name.as_ref()
    }

    pub fn uri(&self) -> &str {
        self.inner.uri.as_ref()
    }

    pub fn desc(&self) -> &str {
        self.inner.desc.as_ref()
    }

    pub fn rename(&mut self, naming: &'static NamingConvention) {
        if self.naming == naming {
            return;
        }
        self.name = (naming.uri_name)(self.name.as_ref());
        self.naming = naming;
    }
}

impl From<UriDefInner> for UriDef {
    fn from(inner: UriDefInner) -> Self {
        let name = inner
            .uri
            .trim_start_matches('.')
            .chars()
            .map(|c| match c {
                '.' => '_',
                c => c.to_ascii_uppercase(),
            })
            .collect();
        Self {
            inner,
            name,
            naming: default_naming(),
        }
    }
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

#[derive(Debug)]
pub struct UriValidationError {
    pub invalid: char,
    pub offset: usize,
    pub reason: &'static str,
}

impl UriValidationError {
    #[inline]
    fn new(invalid: u8, offset: usize, reason: &'static str) -> Self {
        Self {
            invalid: invalid as char,
            reason,
            offset,
        }
    }

    pub fn message_with_uri(&self, uri: &str) -> String {
        format!(
            "invalid uri `{}` at char `{}` (reason: {}, offset {})",
            uri, self.invalid, self.reason, self.offset
        )
    }
}

#[inline]
pub fn validate_bytes(uri: &[u8]) -> Result<UriParts, UriValidationError> {
    let mut prev_char = 0;
    let mut segment_count = 0;
    let mut wildcard_count = 0;
    for (i, c) in uri.iter().copied().enumerate() {
        match c {
            WILDCARD => {
                if prev_char == WILDCARD || wildcard_count == u8::max_value() {
                    return Err(UriValidationError::new(c, i, "double `*`"));
                }
                wildcard_count += 1;
            }
            SEGMENT => {
                if prev_char == SEGMENT || segment_count == u8::max_value() {
                    return Err(UriValidationError::new(c, i, "double `.`"));
                }
                segment_count += 1;
            }
            b'_' | b'a'..=b'z' | b'0'..=b'9' => (),
            _ => return Err(UriValidationError::new(c, i, "invalid char")),
        }
        prev_char = c;
    }
    Ok(UriParts {
        segment_count,
        wildcard_count,
    })
}
