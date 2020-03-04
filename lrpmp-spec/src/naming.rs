use std::cmp::PartialEq;
use std::fmt;

pub use inflector::cases::{
    pascalcase::to_pascal_case, screamingsnakecase::to_screaming_snake_case,
    snakecase::to_snake_case,
};

pub struct NamingConvention {
    pub name: &'static str,
    pub msg_name: fn(&str) -> String,
    pub msg_type: fn(&str) -> String,
    pub msg_field_name: fn(&str) -> String,
    pub msg_field_type: fn(&str) -> String,
}

impl PartialEq<Self> for NamingConvention {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name
    }
}

impl fmt::Debug for NamingConvention {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "NamingConvention({})", self.name)
    }
}

pub(crate) fn default_naming() -> &'static NamingConvention {
    DEFAULT_NAMING_CONVENTION
}

pub(crate) fn unreachable_str_string(_: &str) -> String {
    unreachable!()
}

pub(crate) const DEFAULT_NAMING_CONVENTION: &NamingConvention = &NamingConvention {
    name: "default",
    msg_name: unreachable_str_string,
    msg_type: unreachable_str_string,
    msg_field_name: unreachable_str_string,
    msg_field_type: unreachable_str_string,
};

pub const RUST_NAMING_CONVENTION: &NamingConvention = &NamingConvention {
    name: "rust",
    msg_name: to_pascal_case,
    msg_type: to_pascal_case,
    msg_field_name: to_snake_case,
    msg_field_type: to_pascal_case,
};
