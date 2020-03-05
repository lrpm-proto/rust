mod message;
mod validation;

pub mod naming;
pub mod uri;

use std::fs;
use std::path::Path;
use std::str::FromStr;
use std::sync::Arc;

use semver::Version;
use serde::Deserialize;

use self::naming::{default_naming, NamingConvention};

pub use self::message::*;
pub use self::uri::UriDef;
pub use crate::errors::Error;

pub mod errors {
    use error_chain::error_chain;

    error_chain! {
        foreign_links {
            Io(::std::io::Error);
            Toml(::toml::de::Error);
        }
        errors {
            NoDefaultSpec
        }
    }
}

#[cfg(feature = "default-spec")]
const DEFAULT_SPEC_STR: &str = include_str!("../spec/src/definitions.toml");

#[derive(Debug, Clone, Deserialize)]
struct SpecInner {
    version: Version,
    messages: Vec<MsgDef>,
    uri_definitions: Vec<UriDef>,
    #[serde(default = "default_naming", skip)]
    naming: &'static NamingConvention,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(from = "SpecInner")]
pub struct Spec {
    inner: Arc<SpecInner>,
}

impl Spec {
    #[cfg(feature = "default-spec")]
    pub fn current() -> Result<Self, Error> {
        DEFAULT_SPEC_STR.parse()
    }

    #[cfg(not(feature = "default-spec"))]
    pub fn current() -> Result<Self, Error> {
        Err(self::errors::ErrorKind::NoDefaultSpec.into())
    }

    pub fn load<P>(path: P) -> Result<Self, Error>
    where
        P: AsRef<Path>,
    {
        let path = path.as_ref().canonicalize()?;
        fs::read_to_string(path)?.parse()
    }

    pub fn version(&self) -> &Version {
        &self.inner.version
    }

    pub fn uri_iter(&self) -> impl ExactSizeIterator<Item = &UriDef> {
        self.inner.uri_definitions.iter()
    }

    pub fn message_iter(&self) -> impl ExactSizeIterator<Item = &MsgDef> {
        self.inner.messages.iter()
    }

    pub fn validate(self) -> Result<Self, Error> {
        self::validation::run(self)
    }

    /// Recursively renames names and types given a naming convention.
    pub fn rename(self, naming: &'static NamingConvention) -> Self {
        if self.inner.naming == naming {
            return self;
        }
        let mut spec_inner = (*self.inner).clone();
        for msg in spec_inner.messages.iter_mut() {
            msg.rename(naming);
        }
        spec_inner.naming = naming;
        spec_inner.into()
    }
}

impl From<SpecInner> for Spec {
    fn from(inner: SpecInner) -> Self {
        Self {
            inner: Arc::new(inner),
        }
    }
}

impl FromStr for Spec {
    type Err = Error;

    fn from_str(spec_str: &str) -> Result<Self, Self::Err> {
        Ok(toml::from_str(spec_str)?)
    }
}

#[cfg(test)]
mod tests {
    use super::naming::RUST_NAMING_CONVENTION;
    use super::*;

    #[test]
    fn test_current_spec_valid() {
        Spec::current().unwrap().validate().unwrap();
    }

    #[test]
    fn test_current_spec_rust_valid() {
        Spec::current()
            .unwrap()
            .rename(RUST_NAMING_CONVENTION)
            .validate()
            .unwrap();
    }
}
