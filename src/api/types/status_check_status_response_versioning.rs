pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct CheckStatusResponseVersioning {
    /// The version to build against
    #[serde(default)]
    pub current: String,
    /// Versions that are deprecated but still serving
    #[serde(default)]
    pub deprecated: Vec<String>,
    /// The published versioning and deprecation policy
    #[serde(default)]
    pub policy_url: String,
    /// The headers a deprecated version sends
    #[serde(default)]
    pub signalling: String,
    /// Every version still serving requests
    #[serde(default)]
    pub supported: Vec<String>,
}

impl CheckStatusResponseVersioning {
    pub fn builder() -> CheckStatusResponseVersioningBuilder {
        <CheckStatusResponseVersioningBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CheckStatusResponseVersioningBuilder {
    current: Option<String>,
    deprecated: Option<Vec<String>>,
    policy_url: Option<String>,
    signalling: Option<String>,
    supported: Option<Vec<String>>,
}

impl CheckStatusResponseVersioningBuilder {
    pub fn current(mut self, value: impl Into<String>) -> Self {
        self.current = Some(value.into());
        self
    }

    pub fn deprecated(mut self, value: Vec<String>) -> Self {
        self.deprecated = Some(value);
        self
    }

    pub fn policy_url(mut self, value: impl Into<String>) -> Self {
        self.policy_url = Some(value.into());
        self
    }

    pub fn signalling(mut self, value: impl Into<String>) -> Self {
        self.signalling = Some(value.into());
        self
    }

    pub fn supported(mut self, value: Vec<String>) -> Self {
        self.supported = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`CheckStatusResponseVersioning`].
    /// This method will fail if any of the following fields are not set:
    /// - [`current`](CheckStatusResponseVersioningBuilder::current)
    /// - [`deprecated`](CheckStatusResponseVersioningBuilder::deprecated)
    /// - [`policy_url`](CheckStatusResponseVersioningBuilder::policy_url)
    /// - [`signalling`](CheckStatusResponseVersioningBuilder::signalling)
    /// - [`supported`](CheckStatusResponseVersioningBuilder::supported)
    pub fn build(self) -> Result<CheckStatusResponseVersioning, BuildError> {
        Ok(CheckStatusResponseVersioning {
            current: self
                .current
                .ok_or_else(|| BuildError::missing_field("current"))?,
            deprecated: self
                .deprecated
                .ok_or_else(|| BuildError::missing_field("deprecated"))?,
            policy_url: self
                .policy_url
                .ok_or_else(|| BuildError::missing_field("policy_url"))?,
            signalling: self
                .signalling
                .ok_or_else(|| BuildError::missing_field("signalling"))?,
            supported: self
                .supported
                .ok_or_else(|| BuildError::missing_field("supported"))?,
        })
    }
}
