use std::str::FromStr;

use serde::Deserialize;

use crate::errors::Error;

const SPEC_STR: &str = include_str!("../../spec/src/definitions.toml");

pub mod errors {
    use error_chain::error_chain;

    error_chain! {
        foreign_links {
            Io(::std::io::Error);
            Toml(::toml::de::Error);
        }        
    }
}

#[derive(Debug, Deserialize)]
pub struct Spec {
    pub messages: Vec<MsgDef>,
}

impl Spec {
    pub fn validate(&self) -> Result<(), Error> {
        // TODO
        Ok(())
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

#[derive(Debug, Deserialize)]
pub struct MsgDef {
    pub code: u8,
    pub name: String,
    #[serde(rename = "type")]
    pub ty: String,
    pub stages: Vec<String>,
    pub desc: String,
    pub fields: Vec<MsgFieldDef>,
}

#[derive(Debug, Deserialize)]
pub struct MsgFieldDef {
    pub name: String,
    #[serde(rename = "type")]
    pub ty: String,
    pub desc: String,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_spec_valid() {
        Spec::default().validate().expect("invalid default spec");
    }
}