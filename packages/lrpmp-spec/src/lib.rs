#[cfg(codegen)]
pub mod codegen;

use std::str::FromStr;

use inflector::cases::{pascalcase::to_pascal_case, snakecase::to_snake_case};
use serde::Deserialize;

use crate::errors::Error;

const SPEC_STR: &str = include_str!("../spec/src/definitions.toml");

fn renamed_default() -> bool {
    false
}

pub mod errors {
    use error_chain::error_chain;

    error_chain! {
        foreign_links {
            Io(::std::io::Error);
            Toml(::toml::de::Error);
        }
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct Spec {
    pub messages: Vec<MsgDef>,
    #[serde(default = "renamed_default")]
    renamed: bool,
}

impl Spec {
    pub fn validate(&self) -> Result<(), Error> {
        // TODO
        Ok(())
    }

    /// Recursively renames names and types for use in rust codegen.
    pub fn rust_rename(mut self) -> Self {
        if self.renamed {
            return self;
        }
        for msg in self.messages.iter_mut() {
            msg.rust_rename();
        }
        self.renamed = true;
        self
    }
}

impl FromStr for Spec {
    type Err = Error;

    fn from_str(spec_str: &str) -> Result<Self, Self::Err> {
        Ok(toml::from_str(spec_str)?)
    }
}

impl Default for Spec {
    fn default() -> Self {
        SPEC_STR.parse().expect("invalid spec string")
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct MsgDef {
    pub code: u8,
    pub name: String,
    #[serde(rename = "type")]
    pub ty: String,
    pub stages: Vec<String>,
    pub desc: String,
    pub fields: Vec<MsgFieldDef>,
    #[serde(default = "renamed_default")]
    renamed: bool,
}

impl MsgDef {
    pub(crate) fn rust_rename(&mut self) {
        if self.renamed {
            return;
        }
        self.name = to_pascal_case(self.name.as_ref());
        self.ty = to_snake_case(self.ty.as_ref());
        self.renamed = true;
        for field in self.fields.iter_mut() {
            field.rust_rename();
        }
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct MsgFieldDef {
    pub name: String,
    #[serde(rename = "type")]
    pub ty: String,
    pub desc: String,
    #[serde(default = "renamed_default")]
    renamed: bool,
}

impl MsgFieldDef {
    pub(crate) fn rust_rename(&mut self) {
        if self.renamed {
            return;
        }
        self.renamed = true;
        self.name = to_snake_case(self.name.as_ref());
        self.ty = to_pascal_case(self.ty.as_ref());
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_spec_valid() {
        Spec::default().validate().expect("invalid default spec");
    }
}
