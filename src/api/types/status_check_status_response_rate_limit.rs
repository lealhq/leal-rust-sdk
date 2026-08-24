pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct CheckStatusResponseRateLimit {
    /// Requests allowed per window
    #[serde(default)]
    pub limit: i64,
    /// What the limit is counted against
    #[serde(default)]
    pub scope: String,
    /// Length of the window in seconds
    #[serde(default)]
    pub window_seconds: i64,
}

impl CheckStatusResponseRateLimit {
    pub fn builder() -> CheckStatusResponseRateLimitBuilder {
        <CheckStatusResponseRateLimitBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CheckStatusResponseRateLimitBuilder {
    limit: Option<i64>,
    scope: Option<String>,
    window_seconds: Option<i64>,
}

impl CheckStatusResponseRateLimitBuilder {
    pub fn limit(mut self, value: i64) -> Self {
        self.limit = Some(value);
        self
    }

    pub fn scope(mut self, value: impl Into<String>) -> Self {
        self.scope = Some(value.into());
        self
    }

    pub fn window_seconds(mut self, value: i64) -> Self {
        self.window_seconds = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`CheckStatusResponseRateLimit`].
    /// This method will fail if any of the following fields are not set:
    /// - [`limit`](CheckStatusResponseRateLimitBuilder::limit)
    /// - [`scope`](CheckStatusResponseRateLimitBuilder::scope)
    /// - [`window_seconds`](CheckStatusResponseRateLimitBuilder::window_seconds)
    pub fn build(self) -> Result<CheckStatusResponseRateLimit, BuildError> {
        Ok(CheckStatusResponseRateLimit {
            limit: self
                .limit
                .ok_or_else(|| BuildError::missing_field("limit"))?,
            scope: self
                .scope
                .ok_or_else(|| BuildError::missing_field("scope"))?,
            window_seconds: self
                .window_seconds
                .ok_or_else(|| BuildError::missing_field("window_seconds"))?,
        })
    }
}
