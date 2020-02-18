use std::borrow::Cow;
use std::collections::HashMap;

use serde::{Deserialize, Serialize};

type InnerMap<V> = HashMap<String, V>;

#[derive(Debug, Clone)]
pub enum BasicTypeRef<'a, V> {
    U8(u8),
    U64(u64),
    Str(Cow<'a, str>),
    Map(&'a Map<V>),
    Val(&'a V),
}

pub trait AsBasicTypeRef<'a, V> {
    fn as_basic_type_ref(&'a self) -> BasicTypeRef<'a, V>;
}

impl<'a, T, V> AsBasicTypeRef<'a, V> for &'a T
where
    T: AsBasicTypeRef<'a, V>,
{
    fn as_basic_type_ref(&'a self) -> BasicTypeRef<'a, V> {
        (*self).as_basic_type_ref()
    }
}

/// Represents a single request unique within a session.
#[derive(Debug, Clone)]
pub struct Id(u64);

impl<'a, V> AsBasicTypeRef<'a, V> for Id {
    fn as_basic_type_ref(&'a self) -> BasicTypeRef<'a, V> {
        BasicTypeRef::U64(self.0)
    }
}

/// A `Str` key to `Val` structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Map<V>(InnerMap<V>);

impl<'a, V> AsBasicTypeRef<'a, V> for Map<V> {
    fn as_basic_type_ref(&'a self) -> BasicTypeRef<'a, V> {
        BasicTypeRef::Map(&self)
    }
}

/// Represents a resource unique across all sessions.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Uri(String);

impl<'a, V> AsBasicTypeRef<'a, V> for Uri {
    fn as_basic_type_ref(&'a self) -> BasicTypeRef<'a, V> {
        BasicTypeRef::Str(self.0.as_str().into())
    }
}

/// Represents a resource unique across all sessions.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UriRef<'a>(Cow<'a, str>);

impl<'a, V> AsBasicTypeRef<'a, V> for UriRef<'a> {
    fn as_basic_type_ref(&'a self) -> BasicTypeRef<'a, V> {
        BasicTypeRef::Str(self.0.as_ref().into())
    }
}

/// Represents a message kind (eg, `CALL`, `20`).
#[derive(Debug, Clone, PartialEq)]
pub enum Kind {
    Std(StandardKind),
    Str(Cow<'static, str>),
    Other(u8),
}

impl<'a, V> AsBasicTypeRef<'a, V> for Kind {
    fn as_basic_type_ref(&'a self) -> BasicTypeRef<'a, V> {
        match self {
            Self::Std(s) => s.as_basic_type_ref(),
            Self::Str(s) => BasicTypeRef::Str(s.as_ref().into()),
            Self::Other(o) => BasicTypeRef::U8(*o),
        }
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

/// An arbitrary map of additional information.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Meta<V>(Map<V>);

impl<'a, V> AsBasicTypeRef<'a, V> for Meta<V> {
    fn as_basic_type_ref(&'a self) -> BasicTypeRef<'a, V> {
        BasicTypeRef::Map(&self.0)
    }
}

/// Application specific value.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Body<V>(V);

impl<'a, V> AsBasicTypeRef<'a, V> for Body<V> {
    fn as_basic_type_ref(&'a self) -> BasicTypeRef<'a, V> {
        BasicTypeRef::Val(&self.0)
    }
}
