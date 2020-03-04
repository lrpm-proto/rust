mod message;

pub mod naming;
pub mod uri;

use std::str::FromStr;

use semver::Version;
use serde::Deserialize;

use crate::errors::Error;
use crate::naming::{default_naming, NamingConvention};

pub use self::message::*;

pub mod errors {
    use error_chain::error_chain;

    error_chain! {
        foreign_links {
            Io(::std::io::Error);
            Toml(::toml::de::Error);
        }
    }
}

const SPEC_STR: &str = include_str!("../spec/src/definitions.toml");

#[derive(Debug, Clone, Deserialize)]
pub struct Spec {
    version: Version,
    messages: Vec<MsgDef>,
    #[serde(default = "default_naming", skip)]
    naming: &'static NamingConvention,
}

impl Spec {
    pub fn version(&self) -> &Version {
        &self.version
    }

    pub fn message_iter(&self) -> impl ExactSizeIterator<Item = &MsgDef> {
        self.messages.iter()
    }

    pub fn validate(&self) -> Result<(), Error> {
        // TODO
        Ok(())
    }

    /// Recursively renames names and types given a naming convention.
    pub fn rename(mut self, naming: &'static NamingConvention) -> Self {
        if self.naming == naming {
            return self;
        }
        for msg in self.messages.iter_mut() {
            msg.rename(naming);
        }
        self.naming = naming;
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_spec_valid() {
        Spec::default().validate().expect("invalid default spec");
    }
}
