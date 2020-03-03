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

impl ParseUriError {
    pub(crate) fn new(c: u8, offset: usize) -> Self {
        let invalid = c as char;
        Self { invalid, offset }
    }
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

pub(crate) fn panic_with_expected_type<B>(got: &B, expected: BasicType) -> !
where
    B: BasicValue,
{
    panic!("expected {:?}, got {:?}", expected, got.ty())
}
