use std::borrow::Cow;

use serde::{Deserialize, Serialize};

use super::basic::*;

///////////////////////////////////////////////////////////////////////////////
// Id

/// Represents a single request unique within a session.
#[derive(Debug, Clone)]
pub struct Id(u64);

impl<'a, V> AsBasicTypeRef<'a, V> for Id {
    fn as_basic_type_ref(&'a self) -> BasicTypeRef<'a, V> {
        BasicTypeRef::U64(self.0)
    }
}

///////////////////////////////////////////////////////////////////////////////
// Uri

/// Represents a resource unique across all sessions.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Uri(Cow<'static, str>);

impl<'a, V> AsBasicTypeRef<'a, V> for Uri {
    fn as_basic_type_ref(&'a self) -> BasicTypeRef<'a, V> {
        BasicTypeRef::Str(self.0.as_ref().into())
    }
}

///////////////////////////////////////////////////////////////////////////////
// Meta

/// An arbitrary map of additional information.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Meta<V>(Map<V>);

impl<'a, V> AsBasicTypeRef<'a, V> for Meta<V> {
    fn as_basic_type_ref(&'a self) -> BasicTypeRef<'a, V> {
        BasicTypeRef::Map(&self.0)
    }
}

///////////////////////////////////////////////////////////////////////////////
// Body

/// Application specific value.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Body<V>(V);

impl<'a, V> AsBasicTypeRef<'a, V> for Body<V> {
    fn as_basic_type_ref(&'a self) -> BasicTypeRef<'a, V> {
        BasicTypeRef::Val(&self.0)
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

impl<'a, V> AsBasicTypeRef<'a, V> for Kind {
    fn as_basic_type_ref(&'a self) -> BasicTypeRef<'a, V> {
        match self {
            Self::Known(k) => k.as_basic_type_ref(),
            Self::Unknown(k) => k.as_basic_type_ref(),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum UnknownKind {
    Str(Cow<'static, str>),
    Code(u8),
}

impl<'a, V> AsBasicTypeRef<'a, V> for UnknownKind {
    fn as_basic_type_ref(&'a self) -> BasicTypeRef<'a, V> {
        match self {
            Self::Str(s) => BasicTypeRef::Str(s.as_ref()),
            Self::Code(c) => BasicTypeRef::U8(*c),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum KnownKind {
    Standard(StandardKind),
    Custom(CustomKind),
}

impl<'a, V> AsBasicTypeRef<'a, V> for KnownKind {
    fn as_basic_type_ref(&'a self) -> BasicTypeRef<'a, V> {
        match self {
            Self::Standard(k) => k.as_basic_type_ref(),
            Self::Custom(k) => BasicTypeRef::U8(k.code),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct CustomKind {
    name: &'static str,
    code: u8,
}

impl CustomKind {
    pub const fn new(name: &'static str, code: u8) -> Self {
        Self { name, code }
    }

    pub fn code(&self) -> u8 {
        self.code
    }

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
    pub fn to_str(&self) -> &'static str {
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

impl<'a, V> AsBasicTypeRef<'a, V> for StandardKind {
    fn as_basic_type_ref(&'a self) -> BasicTypeRef<'a, V> {
        BasicTypeRef::U8(self.to_u8())
    }
}
