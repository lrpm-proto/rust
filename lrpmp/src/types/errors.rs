use std::convert::Infallible;

use super::*;

/// Error produced from a invalid basic type conversion.
#[derive(Debug, Clone, PartialEq)]
pub struct UnexpectedType {
    pub expected: &'static [BasicType],
    pub actual: BasicType,
}

impl From<UnexpectedType> for Infallible {
    fn from(_: UnexpectedType) -> Self {
        unreachable!()
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct ParseUriError {
    pub invalid: char,
    pub offset: usize,
}

#[derive(Debug, Clone, PartialEq)]
pub enum UriFromBasicError {
    Parse(ParseUriError),
    UnexpectedType(UnexpectedType),
}

impl From<ParseUriError> for UriFromBasicError {
    fn from(err: ParseUriError) -> Self {
        Self::Parse(err)
    }
}

impl From<UnexpectedType> for UriFromBasicError {
    fn from(err: UnexpectedType) -> Self {
        Self::UnexpectedType(err)
    }
}

///////////////////////////////////////////////////////////////////////////////

#[derive(Debug, Clone, PartialEq)]
pub enum KnownKindFromBasicError {
    UnknownKind(UnknownKind),
    UnexpectedType(UnexpectedType),
}

impl From<UnknownKind> for KnownKindFromBasicError {
    fn from(kind: UnknownKind) -> Self {
        Self::UnknownKind(kind)
    }
}

impl From<UnexpectedType> for KnownKindFromBasicError {
    fn from(err: UnexpectedType) -> Self {
        Self::UnexpectedType(err)
    }
}
