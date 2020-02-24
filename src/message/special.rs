use std::borrow::Cow;
use std::convert::TryFrom;
use std::fmt;

use bytes::Bytes;
use bytestring::ByteString;
use serde::{Deserialize, Serialize};

use super::basic::*;

///////////////////////////////////////////////////////////////////////////////
// Id

/// Represents a single request unique within a session.
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct Id(u64);

impl Id {
    pub const fn new(value: u64) -> Self {
        Self(value)
    }
}

impl<'a, V> AsBasicValueRef<'a, V> for Id {
    fn as_basic_value_ref(&'a self) -> BasicValueRef<'a, V> {
        BasicValueRef::U64(self.0)
    }
}

impl<V> TryFrom<BasicValue<V>> for Id {
    type Error = UnexpectedBasicTypeError;

    fn try_from(value: BasicValue<V>) -> Result<Self, Self::Error> {
        match value {
            BasicValue::U64(v) => Ok(Self::new(v)),
            other => Err(UnexpectedBasicTypeError {
                expected: &[BasicType::U64],
                actual: other.ty(),
            }),
        }
    }
}

impl From<Id> for u64 {
    fn from(id: Id) -> Self {
        id.0
    }
}

///////////////////////////////////////////////////////////////////////////////
// Uri

#[derive(Debug, Clone, PartialEq)]
pub struct ParseUriError {
    pub invalid: char,
    pub offset: usize,
}

impl ParseUriError {
    fn new(c: u8, offset: usize) -> Self {
        let invalid = c as char;
        Self { invalid, offset }
    }
}

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

impl<'a, V> AsBasicValueRef<'a, V> for Uri {
    fn as_basic_value_ref(&'a self) -> BasicValueRef<'a, V> {
        BasicValueRef::Str(self.as_str())
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum ParseBasicUriError {
    Parse(ParseUriError),
    UnexpectedBasicType(UnexpectedBasicTypeError),
}

impl<V> TryFrom<BasicValue<V>> for Uri {
    type Error = ParseBasicUriError;

    fn try_from(value: BasicValue<V>) -> Result<Self, Self::Error> {
        match value {
            BasicValue::Str(v) => Self::try_from(v).map_err(ParseBasicUriError::Parse),
            other => Err(ParseBasicUriError::UnexpectedBasicType(
                UnexpectedBasicTypeError {
                    expected: &[BasicType::U64],
                    actual: other.ty(),
                },
            )),
        }
    }
}

///////////////////////////////////////////////////////////////////////////////
// Meta

/// An arbitrary map of additional information.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Meta<V>(Map<V>);

impl<'a, V> AsBasicValueRef<'a, V> for Meta<V> {
    fn as_basic_value_ref(&'a self) -> BasicValueRef<'a, V> {
        BasicValueRef::Map(&self.0)
    }
}

///////////////////////////////////////////////////////////////////////////////
// Body

/// Application specific value.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Body<V>(V);

impl<'a, V> AsBasicValueRef<'a, V> for Body<V> {
    fn as_basic_value_ref(&'a self) -> BasicValueRef<'a, V> {
        BasicValueRef::Val(&self.0)
    }
}

///////////////////////////////////////////////////////////////////////////////
// Kind

/// Represents a message kind (eg, `CALL`, `20`).
#[derive(Debug, Clone, PartialEq)]
pub enum Kind {
    Known(KnownKind),
    Unknown(UnknownKind),
}

impl<'a, V> AsBasicValueRef<'a, V> for Kind {
    fn as_basic_value_ref(&'a self) -> BasicValueRef<'a, V> {
        match self {
            Self::Known(k) => k.as_basic_value_ref(),
            Self::Unknown(k) => k.as_basic_value_ref(),
        }
    }
}

/// Represents an unknown message kind.
#[derive(Debug, Clone, PartialEq)]
pub enum UnknownKind {
    Str(Cow<'static, str>),
    Code(u8),
}

impl<'a, V> AsBasicValueRef<'a, V> for UnknownKind {
    fn as_basic_value_ref(&'a self) -> BasicValueRef<'a, V> {
        match self {
            Self::Str(s) => BasicValueRef::Str(s.as_ref()),
            Self::Code(c) => BasicValueRef::U8(*c),
        }
    }
}

/// Represents a defined message kind.
#[derive(Debug, Clone, PartialEq)]
pub enum KnownKind {
    Standard(StandardKind),
    Custom(CustomKind),
}

impl<'a, V> AsBasicValueRef<'a, V> for KnownKind {
    fn as_basic_value_ref(&'a self) -> BasicValueRef<'a, V> {
        match self {
            Self::Standard(k) => k.as_basic_value_ref(),
            Self::Custom(k) => BasicValueRef::U8(k.code),
        }
    }
}

/// A custom defined message kind.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct CustomKind {
    name: &'static str,
    code: u8,
}

impl CustomKind {
    /// Constructs a new custom kind.
    pub const fn new(name: &'static str, code: u8) -> Self {
        Self { name, code }
    }

    /// Returns the kind code.
    pub fn code(&self) -> u8 {
        self.code
    }

    /// Returns the kind name.
    pub fn name(&self) -> &'static str {
        self.name
    }
}

/// Standard defined message kinds.
#[derive(Debug, Clone, Copy, PartialEq)]
#[repr(u8)]
pub enum StandardKind {
    // Init
    Goodbye = 1,
    Hello = 2,
    Prove = 3,
    Proof = 4,
    // Generic
    Error = 20,
    Cancel = 21,
    // RPC
    Call = 40,
    Result = 41,
    // PubSub
    Event = 60,
    Publish = 61,
    Published = 62,
    Subscribe = 63,
    Subscribed = 64,
    Unsubscribe = 65,
    Unsubscribed = 66,
}

impl StandardKind {
    pub fn to_str(self) -> &'static str {
        match self {
            Self::Goodbye => "GOODBYE",
            Self::Hello => "HELLO",
            Self::Prove => "PROVE",
            Self::Proof => "PROOF",
            Self::Error => "ERROR",
            Self::Cancel => "CANCEL",
            Self::Call => "CALL",
            Self::Result => "RESULT",
            Self::Event => "EVENT",
            Self::Publish => "PUBLISH",
            Self::Published => "PUBLISHED",
            Self::Subscribe => "SUBSCRIBE",
            Self::Subscribed => "SUBSCRIBED",
            Self::Unsubscribe => "UNSUBSCRIBE",
            Self::Unsubscribed => "UNSUBSCRIBED",
        }
    }

    pub fn to_u8(self) -> u8 {
        self.into()
    }
}

impl From<StandardKind> for u8 {
    fn from(kind: StandardKind) -> u8 {
        kind as u8
    }
}

impl<'a, V> AsBasicValueRef<'a, V> for StandardKind {
    fn as_basic_value_ref(&'a self) -> BasicValueRef<'a, V> {
        BasicValueRef::U8(self.to_u8())
    }
}
