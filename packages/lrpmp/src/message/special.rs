use std::convert::TryFrom;
use std::fmt;

use bytes::Bytes;
use bytestring::ByteString;
use serde::{Deserialize, Serialize};

use super::basic::*;

pub use super::standard::StandardKind;

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

impl<V, M> BasicValue<V, M> for Id {
    fn ty(&self) -> BasicType {
        BasicType::U64
    }

    fn as_u64(&self) -> u64 {
        self.0
    }

    impl_invalid_basic_types!(<V, M>, U8, Str, Map, Val);
}

impl<V> FromBasicValue<V> for Id {
    type Error = UnexpectedType;

    fn expected_types() -> &'static [BasicType] {
        &[BasicType::Map]
    }

    fn from_basic_value<B>(value: B) -> Result<Self, Self::Error>
    where
        B: BasicValue<V>,
    {
        Ok(Self(value.try_as_u64()?))
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

#[derive(Debug, Clone, PartialEq)]
pub enum ParseBasicUriError {
    Parse(ParseUriError),
    UnexpectedType(UnexpectedType),
}

impl From<ParseUriError> for ParseBasicUriError {
    fn from(err: ParseUriError) -> Self {
        Self::Parse(err)
    }
}

impl From<UnexpectedType> for ParseBasicUriError {
    fn from(err: UnexpectedType) -> Self {
        Self::UnexpectedType(err)
    }
}

impl<V, M> BasicValue<V, M> for Uri {
    fn ty(&self) -> BasicType {
        BasicType::Str
    }

    fn as_str(&self) -> &str {
        self.as_str()
    }

    fn into_string(self) -> ByteString {
        self.contents
    }

    impl_invalid_basic_types!(<V, M>, U8, U64, Map, Val);
}

impl<V> FromBasicValue<V> for Uri {
    type Error = ParseBasicUriError;

    fn expected_types() -> &'static [BasicType] {
        &[BasicType::Str]
    }

    fn from_basic_value<B>(value: B) -> Result<Self, Self::Error>
    where
        B: BasicValue<V>,
    {
        Ok(Self::try_from(value.try_into_string()?)?)
    }
}

///////////////////////////////////////////////////////////////////////////////
// Meta

/// An arbitrary map of additional information.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Meta<V>(Map<V>);

impl<V> Meta<V> {
    pub fn new() -> Self {
        Self(Map::new())
    }
}

impl<V> BasicValue<V> for Meta<V> {
    fn ty(&self) -> BasicType {
        BasicType::Str
    }

    fn as_map(&self) -> &Map<V> {
        &self.0
    }

    fn into_map(self) -> Map<V> {
        self.0
    }

    impl_invalid_basic_types!(<V, Map<V>>, U8, U64, Str, Val);
}

impl<V> FromBasicValue<V> for Meta<V> {
    type Error = UnexpectedType;

    fn expected_types() -> &'static [BasicType] {
        &[BasicType::Map]
    }

    fn from_basic_value<B>(value: B) -> Result<Self, Self::Error>
    where
        B: BasicValue<V>,
    {
        Ok(Self(value.try_into_map()?))
    }
}

impl<V> Default for Meta<V> {
    fn default() -> Self {
        Self::new()
    }
}

///////////////////////////////////////////////////////////////////////////////
// Body

/// Application specific value.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Body<V>(V);

impl<V> Body<V> {
    pub fn new(value: V) -> Self {
        Self(value)
    }
}

impl<V, M> BasicValue<V, M> for Body<V> {
    fn ty(&self) -> BasicType {
        BasicType::Str
    }

    fn as_val(&self) -> &V {
        &self.0
    }

    fn into_val(self) -> V {
        self.0
    }

    impl_invalid_basic_types!(<V, M>, U8, U64, Str, Map);
}

impl<V> FromBasicValue<V> for Body<V> {
    type Error = UnexpectedType;

    fn expected_types() -> &'static [BasicType] {
        &[BasicType::Val]
    }

    fn from_basic_value<B>(value: B) -> Result<Self, Self::Error>
    where
        B: BasicValue<V>,
    {
        Ok(Self(value.try_into_val()?))
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

impl<V, M> BasicValue<V, M> for Kind {
    fn ty(&self) -> BasicType {
        match self {
            Kind::Known(_) => BasicType::U8,
            Kind::Unknown(UnknownKind::Code(_)) => BasicType::U8,
            Kind::Unknown(UnknownKind::Name(_)) => BasicType::Str,
        }
    }

    fn as_u8(&self) -> u8 {
        match self {
            Kind::Known(k) => k.code(),
            Kind::Unknown(UnknownKind::Code(c)) => *c,
            _ => panic_with_expected_type::<Self, V, M>(&self, BasicType::U8),
        }
    }

    fn as_str(&self) -> &str {
        match self {
            Kind::Unknown(UnknownKind::Name(n)) => n.as_ref(),
            _ => panic_with_expected_type::<Self, V, M>(&self, BasicType::Str),
        }
    }

    fn into_string(self) -> ByteString {
        match self {
            Kind::Unknown(UnknownKind::Name(n)) => n,
            _ => panic_with_expected_type::<Self, V, M>(&self, BasicType::Str),
        }
    }

    impl_invalid_basic_types!(<V, M>, U64, Map, Val);
}

impl<V> FromBasicValue<V> for Kind {
    type Error = UnexpectedType;

    fn expected_types() -> &'static [BasicType] {
        &[BasicType::U8, BasicType::Str]
    }

    fn from_basic_value<B>(value: B) -> Result<Self, Self::Error>
    where
        B: BasicValue<V>,
    {
        match value.ty() {
            BasicType::U8 => Ok(Self::from_code(value.as_u8())),
            BasicType::Str => Ok(Self::from_name(value.into_string())),
            other_ty => Err(UnexpectedType {
                expected: <Self as FromBasicValue<V>>::expected_types(),
                actual: other_ty,
            }),
        }
    }
}

///////////////////////////////////////////////////////////////////////////////

/// Represents an unknown message kind.
#[derive(Debug, Clone, PartialEq)]
pub enum UnknownKind {
    Name(ByteString),
    Code(u8),
}

///////////////////////////////////////////////////////////////////////////////

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
        match self {
            Self::Standard(k) => k.field_count(),
            Self::Custom(k) => k.field_count(),
        }
    }
}

impl From<CustomKind> for KnownKind {
    fn from(kind: CustomKind) -> Self {
        Self::Custom(kind)
    }
}

impl From<StandardKind> for KnownKind {
    fn from(kind: StandardKind) -> Self {
        Self::Standard(kind)
    }
}

pub enum ParseKnownKindError {
    UnknownKind(UnknownKind),
    UnexpectedType(UnexpectedType),
}

impl From<UnexpectedType> for ParseKnownKindError {
    fn from(err: UnexpectedType) -> Self {
        Self::UnexpectedType(err)
    }
}

///////////////////////////////////////////////////////////////////////////////

/// A custom defined message kind.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct CustomKind {
    name: &'static str,
    code: u8,
    fields_min: usize,
    fields_max: Option<usize>,
}

impl CustomKind {
    /// Constructs a new custom kind.
    pub const fn new(
        name: &'static str,
        code: u8,
        fields_min: usize,
        fields_max: Option<usize>,
    ) -> Self {
        Self {
            name,
            code,
            fields_min,
            fields_max,
        }
    }

    /// Returns the kind code.
    pub fn code(&self) -> u8 {
        self.code
    }

    /// Returns the kind name.
    pub fn name(&self) -> &'static str {
        self.name
    }

    /// Returns the lower and upper bound of the number of fields in the message kind.
    pub fn field_count(&self) -> (usize, Option<usize>) {
        (self.fields_min, self.fields_max)
    }
}

///////////////////////////////////////////////////////////////////////////////

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
