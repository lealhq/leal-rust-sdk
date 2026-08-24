pub use crate::prelude::*;

/// A JSON error payload. Agents should read `error` for a human readable summary and `errors` for per field validation messages when present.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct Error {
    /// Human readable description of what went wrong.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
    /// Validation messages, either a list of strings or an object keyed by field name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub errors: Option<ErrorErrors>,
    /// Additional properties that are not part of the defined schema.
    #[serde(flatten)]
    pub extra: std::collections::HashMap<String, serde_json::Value>,
}

impl Error {
    pub fn builder() -> ErrorBuilder {
        <ErrorBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ErrorBuilder {
    error: Option<String>,
    errors: Option<ErrorErrors>,
}

impl ErrorBuilder {
    pub fn error(mut self, value: impl Into<String>) -> Self {
        self.error = Some(value.into());
        self
    }

    pub fn errors(mut self, value: ErrorErrors) -> Self {
        self.errors = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`Error`].
    pub fn build(self) -> Result<Error, BuildError> {
        Ok(Error {
            error: self.error,
            errors: self.errors,
            extra: Default::default(),
        })
    }
}
