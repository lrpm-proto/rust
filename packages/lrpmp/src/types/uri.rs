use std::convert::TryFrom;
use std::fmt;

use bytes::Bytes;
use bytestring::ByteString;

use lrpmp_spec::uri::{UriAnalysis, UriParts};

use super::*;

/// Represents a resource unique across all sessions.
// TODO: Serialize, Deserialize
#[derive(Debug, Clone)]
pub struct Uri {
    contents: ByteString,
    parts: UriParts,
}

impl Uri {
    pub fn as_str(&self) -> &str {
        self.contents.as_ref()
    }

    pub fn has_wildcard(&self) -> bool {
        self.parts.wildcard_count > 0
    }

    pub fn segment_count(&self) -> u8 {
        self.parts.segment_count
    }

    pub fn wildcard_count(&self) -> u8 {
        self.parts.wildcard_count
    }

    pub fn from_static(s: &'static str) -> Result<Self, ParseUriError> {
        Self::try_from(Bytes::from_static(s.as_bytes()))
    }

    pub const unsafe fn from_parts_unchecked(contents: Bytes, parts: UriParts) -> Self {
        let contents = ByteString::from_bytes_unchecked(contents);
        Uri { contents, parts }
    }

    pub const unsafe fn from_static_parts_unchecked(uri: &'static str, parts: UriParts) -> Self {
        Self::from_parts_unchecked(Bytes::from_static(uri.as_bytes()), parts)
    }
}

impl fmt::Display for Uri {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

impl TryFrom<Bytes> for Uri {
    type Error = ParseUriError;

    fn try_from(contents: Bytes) -> Result<Self, Self::Error> {
        match UriAnalysis::for_uri_bytes(contents.as_ref()) {
            UriAnalysis::Valid(parts) => unsafe { Ok(Uri::from_parts_unchecked(contents, parts)) },
            UriAnalysis::Invalid { invalid, offset } => Err(ParseUriError { invalid, offset }),
        }
    }
}

impl TryFrom<String> for Uri {
    type Error = ParseUriError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        Self::try_from(Bytes::from(value))
    }
}

impl TryFrom<ByteString> for Uri {
    type Error = ParseUriError;

    fn try_from(value: ByteString) -> Result<Self, Self::Error> {
        Self::try_from(value.into_inner())
    }
}

impl<M, V> BasicValue<M, V> for Uri {
    fn ty(&self) -> BasicType {
        BasicType::Str
    }

    fn as_str(&self) -> &str {
        self.as_str()
    }

    fn into_string(self) -> String {
        self.contents.to_string()
    }

    impl_invalid_basic_types!(<M, V> U8, U64, Map, Val);
}

impl<M, V> FromBasicValuePart<M, V> for Uri {
    type Error = UriFromBasicError;

    fn expected_types() -> &'static [BasicType] {
        &[BasicType::Str]
    }

    fn from_basic_str(v: String) -> Result<Self, Self::Error> {
        Ok(Self::try_from(v)?)
    }
}
