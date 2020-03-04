use super::*;

pub use crate::std_impl::StandardKind;

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

    pub fn from_name(name: String) -> Self {
        if let Some(known) = KnownKind::from_name(name.as_ref()) {
            return Self::Known(known);
        }
        Self::Unknown(UnknownKind::Name(name))
    }
}

impl<M, V> BasicValue<M, V> for Kind {
    fn ty(&self) -> BasicType {
        match self {
            Kind::Known(_) => BasicType::U8,
            Kind::Unknown(UnknownKind::Code(_)) => BasicType::U8,
            Kind::Unknown(UnknownKind::Name(_)) => BasicType::Str,
        }
    }

    fn as_u8(&self) -> u8 {
        <Self as BasicValueExt<M, V>>::assert_is_type(self, BasicType::U8);
        match self {
            Kind::Known(k) => k.code(),
            Kind::Unknown(UnknownKind::Code(c)) => *c,
            _ => unreachable!(),
        }
    }

    fn as_str(&self) -> &str {
        <Self as BasicValueExt<M, V>>::assert_is_type(self, BasicType::Str);
        match self {
            Kind::Unknown(UnknownKind::Name(n)) => n.as_ref(),
            _ => unreachable!(),
        }
    }

    fn into_string(self) -> String {
        <Self as BasicValueExt<M, V>>::assert_is_type(&self, BasicType::Str);
        match self {
            Kind::Unknown(UnknownKind::Name(n)) => n,
            _ => unreachable!(),
        }
    }

    impl_invalid_basic_types!(<M, V> U64, Map, Val);
}

impl<M, V> FromBasicValuePart<M, V> for Kind {
    type Error = UnexpectedType;

    fn expected_types() -> &'static [BasicType] {
        &[BasicType::U8, BasicType::Str]
    }

    fn from_basic_u8(v: u8) -> Result<Self, Self::Error> {
        Ok(Self::from_code(v))
    }

    fn from_basic_str(v: String) -> Result<Self, Self::Error> {
        Ok(Self::from_name(v))
    }
}

///////////////////////////////////////////////////////////////////////////////

/// Represents an unknown message kind.
#[derive(Debug, Clone, PartialEq)]
pub enum UnknownKind {
    Name(String),
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
    pub fn is_standard(&self) -> bool {
        match self {
            Self::Standard(_) => true,
            Self::Custom(_) => false,
        }
    }

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

impl<M, V> FromBasicValuePart<M, V> for KnownKind {
    type Error = KnownKindFromBasicError;

    fn expected_types() -> &'static [BasicType] {
        &[BasicType::U8, BasicType::Str]
    }

    fn from_basic_u8(v: u8) -> Result<Self, Self::Error> {
        match <Kind as FromBasicValuePart<M, V>>::from_basic_u8(v)? {
            Kind::Known(k) => Ok(k),
            Kind::Unknown(k) => Err(k.into()),
        }
    }

    fn from_basic_str(v: String) -> Result<Self, Self::Error> {
        match <Kind as FromBasicValuePart<M, V>>::from_basic_str(v)? {
            Kind::Known(k) => Ok(k),
            Kind::Unknown(k) => Err(k.into()),
        }
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
