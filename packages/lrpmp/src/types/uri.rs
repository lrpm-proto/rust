use std::convert::TryFrom;
use std::fmt;

use bytes::Bytes;

use super::*;

/// Represents a resource unique across all sessions.
// TODO: Serialize, Deserialize
#[derive(Debug, Clone)]
pub struct Uri {
    contents: ByteString,
    segment_count: u8,
    wildcard_count: u8,
}

impl Uri {
    const SEGMENT: u8 = b'.';
    const WILDCARD: u8 = b'*';

    pub fn as_str(&self) -> &str {
        self.contents.as_ref()
    }

    pub fn has_wildcard(&self) -> bool {
        self.wildcard_count > 0
    }

    pub fn segment_count(&self) -> u8 {
        self.segment_count
    }

    pub fn wildcard_count(&self) -> u8 {
        self.wildcard_count
    }

    pub fn from_static(s: &'static str) -> Result<Self, ParseUriError> {
        Self::from_bytes(Bytes::from_static(s.as_ref()))
    }

    fn from_bytes(contents: Bytes) -> Result<Self, ParseUriError> {
        let mut prev_char = 0;
        let mut segment_count = 0;
        let mut wildcard_count = 0;
        for (i, c) in contents.as_ref().iter().copied().enumerate() {
            match c {
                Self::WILDCARD => {
                    if prev_char == Self::WILDCARD || wildcard_count == u8::max_value() {
                        return Err(ParseUriError::new(c, i));
                    }
                    wildcard_count += 1;
                }
                Self::SEGMENT if prev_char == Self::SEGMENT => {
                    if prev_char == Self::SEGMENT || segment_count == u8::max_value() {
                        return Err(ParseUriError::new(c, i));
                    }
                    segment_count += 1;
                }
                b'_' | b'a'..=b'z' | b'0'..=b'9' => (),
                _ => {
                    return Err(ParseUriError {
                        invalid: c as char,
                        offset: i,
                    })
                }
            }
            prev_char = c;
        }
        let contents = unsafe { ByteString::from_bytes_unchecked(contents) };
        Ok(Self {
            contents,
            segment_count,
            wildcard_count,
        })
    }
}

impl fmt::Display for Uri {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

impl TryFrom<Bytes> for Uri {
    type Error = ParseUriError;

    fn try_from(value: Bytes) -> Result<Self, Self::Error> {
        Self::from_bytes(value)
    }
}

impl TryFrom<String> for Uri {
    type Error = ParseUriError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        Self::from_bytes(Bytes::from(value))
    }
}

impl TryFrom<ByteString> for Uri {
    type Error = ParseUriError;

    fn try_from(value: ByteString) -> Result<Self, Self::Error> {
        Self::from_bytes(value.into_inner())
    }
}

impl BasicValue for Uri {
    type Map = InvalidBasicValue;
    type Val = InvalidBasicValue;

    fn ty(&self) -> BasicType {
        BasicType::Str
    }

    fn as_str(&self) -> &str {
        self.as_str()
    }

    fn into_string(self) -> ByteString {
        self.contents
    }

    impl_invalid_basic_types!(U8, U64, Map, Val);
}

impl<B> FromBasicValue<B> for Uri
where
    B: BasicValue,
{
    type Error = UriFromBasicError;

    fn expected_types() -> &'static [BasicType] {
        &[BasicType::Str]
    }

    fn from_basic_value(value: B) -> Result<Self, Self::Error> {
        Ok(Self::try_from(value.try_into_string()?)?)
    }
}
