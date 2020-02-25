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

impl<V> From<Id> for BasicValue<V> {
    fn from(value: Id) -> Self {
        Self::U64(value.0)
    }
}

impl<'a, V> AsBasicValueRef<'a, V> for Id {
    fn as_basic_value_ref(&'a self) -> BasicValueRef<'a, V> {
        BasicValueRef::U64(self.0)
    }
}

impl<V> FromBasicValue<V> for Id {
    type Error = UnexpectedBasicTypeError;

    fn expected_types() -> &'static [BasicType] {
        &[BasicType::U64]
    }

    fn from_basic_value(value: BasicValue<V>) -> Result<Self, Self::Error> {
        match value {
            BasicValue::U64(v) => Ok(Self::new(v)),
            other => Err(Self::unexpected_error(other)),
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

impl<V> From<Uri> for BasicValue<V> {
    fn from(value: Uri) -> Self {
        Self::Str(value.contents)
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

impl From<UnexpectedBasicTypeError> for ParseBasicUriError {
    fn from(err: UnexpectedBasicTypeError) -> Self {
        Self::UnexpectedBasicType(err)
    }
}

impl<V> FromBasicValue<V> for Uri {
    type Error = ParseBasicUriError;

    fn expected_types() -> &'static [BasicType] {
        &[BasicType::Str]
    }

    fn from_basic_value(value: BasicValue<V>) -> Result<Self, Self::Error> {
        match value {
            BasicValue::Str(v) => Self::try_from(v).map_err(ParseBasicUriError::Parse),
            other => Err(Self::unexpected_error(other)),
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

impl<V> FromBasicValue<V> for Meta<V> {
    type Error = UnexpectedBasicTypeError;

    fn expected_types() -> &'static [BasicType] {
        &[BasicType::Map]
    }

    fn from_basic_value(value: BasicValue<V>) -> Result<Self, Self::Error> {
        match value {
            BasicValue::Map(v) => Ok(Self(v)),
            other => Err(Self::unexpected_error(other)),
        }
    }
}

///////////////////////////////////////////////////////////////////////////////
// Body

/// Application specific value.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Body<V>(V);

impl<V> From<Body<V>> for BasicValue<V> {
    fn from(value: Body<V>) -> Self {
        Self::Val(value.0)
    }
}

impl<'a, V> AsBasicValueRef<'a, V> for Body<V> {
    fn as_basic_value_ref(&'a self) -> BasicValueRef<'a, V> {
        BasicValueRef::Val(&self.0)
    }
}

impl<V> FromBasicValue<V> for Body<V> {
    type Error = UnexpectedBasicTypeError;

    fn expected_types() -> &'static [BasicType] {
        &[BasicType::Val]
    }

    fn from_basic_value(value: BasicValue<V>) -> Result<Self, Self::Error> {
        match value {
            BasicValue::Val(v) => Ok(Self(v)),
            other => Err(Self::unexpected_error(other)),
        }
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

impl Kind {
    pub fn from_code(code: u8) -> Self {
        if let Some(known) = KnownKind::from_code(code) {
            return Self::Known(known);
        }
        Self::Unknown(UnknownKind::Code(code))
    }

    pub fn from_name(name: ByteString) -> Self {
        if let Some(known) = KnownKind::from_name(name.as_ref()) {
            return Self::Known(known);
        }
        Self::Unknown(UnknownKind::Name(name))
    }
}

impl<V> From<Kind> for BasicValue<V> {
    fn from(value: Kind) -> Self {
        match value {
            Kind::Known(k) => k.into(),
            Kind::Unknown(k) => k.into(),
        }
    }
}

impl<'a, V> AsBasicValueRef<'a, V> for Kind {
    fn as_basic_value_ref(&'a self) -> BasicValueRef<'a, V> {
        match self {
            Self::Known(k) => k.as_basic_value_ref(),
            Self::Unknown(k) => k.as_basic_value_ref(),
        }
    }
}

impl<V> FromBasicValue<V> for Kind {
    type Error = UnexpectedBasicTypeError;

    fn expected_types() -> &'static [BasicType] {
        &[BasicType::U8, BasicType::Str]
    }

    fn from_basic_value(value: BasicValue<V>) -> Result<Self, Self::Error> {
        match value {
            BasicValue::U8(v) => Ok(Self::from_code(v)),
            BasicValue::Str(v) => Ok(Self::from_name(v)),
            other => Err(Self::unexpected_error(other)),
        }
    }
}

/// Represents an unknown message kind.
#[derive(Debug, Clone, PartialEq)]
pub enum UnknownKind {
    Name(ByteString),
    Code(u8),
}

impl<V> From<UnknownKind> for BasicValue<V> {
    fn from(value: UnknownKind) -> Self {
        match value {
            UnknownKind::Name(s) => Self::Str(s),
            UnknownKind::Code(c) => Self::U8(c),
        }
    }
}

impl<'a, V> AsBasicValueRef<'a, V> for UnknownKind {
    fn as_basic_value_ref(&'a self) -> BasicValueRef<'a, V> {
        match self {
            Self::Name(s) => BasicValueRef::Str(s.as_ref()),
            Self::Code(c) => BasicValueRef::U8(*c),
        }
    }
}

/// Represents a defined message kind.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum KnownKind {
    Standard(StandardKind),
    Custom(CustomKind),
}

impl KnownKind {
    pub fn code(&self) -> u8 {
        match self {
            Self::Standard(k) => k.code(),
            Self::Custom(k) => k.code(),
        }
    }

    pub fn from_name(name: &str) -> Option<Self> {
        StandardKind::from_name(name).map(Self::Standard)
    }

    pub fn from_code(code: u8) -> Option<Self> {
        StandardKind::from_code(code).map(Self::Standard)
    }

    /// Returns the lower and upper bound of the number of fields in the message kind.
    pub fn field_count(&self) -> (usize, Option<usize>) {
        // TODO
        (0, None)
    }
}

pub enum ParseKnownKindError {
    UnknownKind(UnknownKind),
    UnexpectedType(UnexpectedBasicTypeError),
}

impl From<UnexpectedBasicTypeError> for ParseKnownKindError {
    fn from(err: UnexpectedBasicTypeError) -> Self {
        Self::UnexpectedType(err)
    }
}

impl<V> FromBasicValue<V> for KnownKind {
    type Error = ParseKnownKindError;

    fn expected_types() -> &'static [BasicType] {
        &[BasicType::U8, BasicType::Str]
    }

    fn from_basic_value(value: BasicValue<V>) -> Result<Self, Self::Error> {
        let unknown_kind = match value {
            BasicValue::U8(v) => match Self::from_code(v) {
                Some(v) => return Ok(v),
                None => UnknownKind::Code(v),
            },
            BasicValue::Str(v) => match Self::from_name(v.as_ref()) {
                Some(v) => return Ok(v),
                None => UnknownKind::Name(v),
            },
            other => return Err(Self::unexpected_error(other)),
        };
        Err(ParseKnownKindError::UnknownKind(unknown_kind))
    }
}

impl<V> From<KnownKind> for BasicValue<V> {
    fn from(value: KnownKind) -> Self {
        match value {
            KnownKind::Standard(k) => k.into(),
            KnownKind::Custom(k) => BasicValue::U8(k.code()),
        }
    }
}

impl<'a, V> AsBasicValueRef<'a, V> for KnownKind {
    fn as_basic_value_ref(&'a self) -> BasicValueRef<'a, V> {
        match self {
            Self::Standard(k) => k.as_basic_value_ref(),
            Self::Custom(k) => BasicValueRef::U8(k.code()),
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

impl_standard_kind!(
    // Init
    (Goodbye, "GOODBYE", 1),
    (Hello, "HELLO", 2),
    (Prove, "PROVE", 3),
    (Proof, "PROOF", 4),
    // Generic
    (Error, "ERROR", 20),
    (Cancel, "CANCEL", 21),
    // RPC
    (Call, "CALL", 40),
    (Result, "RESULT", 41),
    // PubSub
    (Event, "EVENT", 60),
    (Publish, "PUBLISH", 61),
    (Published, "PUBLISHED", 62),
    (Subscribe, "SUBSCRIBE", 63),
    (Subscribed, "SUBSCRIBED", 64),
    (Unsubscribe, "UNSUBSCRIBE", 65),
    (Unsubscribed, "UNSUBSCRIBED", 66)
);

impl StandardKind {
    pub fn code(self) -> u8 {
        self.into()
    }
}

impl From<StandardKind> for u8 {
    fn from(kind: StandardKind) -> u8 {
        kind as u8
    }
}

impl<V> From<StandardKind> for BasicValue<V> {
    fn from(value: StandardKind) -> Self {
        BasicValue::U8(value.code())
    }
}

impl<'a, V> AsBasicValueRef<'a, V> for StandardKind {
    fn as_basic_value_ref(&'a self) -> BasicValueRef<'a, V> {
        BasicValueRef::U8(self.code())
    }
}
