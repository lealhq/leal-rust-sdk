pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct CheckStatusResponse {
    /// Current API version
    #[serde(default)]
    pub api_version: String,
    /// How to authenticate a request
    #[serde(default)]
    pub authentication: String,
    /// Developer portal: quickstart, auth, webhooks
    #[serde(default)]
    pub developer_portal_url: String,
    /// Human readable API reference
    #[serde(default)]
    pub documentation_url: String,
    /// OpenAPI description of this API
    #[serde(default)]
    pub openapi_url: String,
    #[serde(default)]
    pub rate_limit: CheckStatusResponseRateLimit,
    /// 'ok' while the API is serving requests
    #[serde(default)]
    pub status: String,
}

impl CheckStatusResponse {
    pub fn builder() -> CheckStatusResponseBuilder {
        <CheckStatusResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CheckStatusResponseBuilder {
    api_version: Option<String>,
    authentication: Option<String>,
    developer_portal_url: Option<String>,
    documentation_url: Option<String>,
    openapi_url: Option<String>,
    rate_limit: Option<CheckStatusResponseRateLimit>,
    status: Option<String>,
}

impl CheckStatusResponseBuilder {
    pub fn api_version(mut self, value: impl Into<String>) -> Self {
        self.api_version = Some(value.into());
        self
    }

    pub fn authentication(mut self, value: impl Into<String>) -> Self {
        self.authentication = Some(value.into());
        self
    }

    pub fn developer_portal_url(mut self, value: impl Into<String>) -> Self {
        self.developer_portal_url = Some(value.into());
        self
    }

    pub fn documentation_url(mut self, value: impl Into<String>) -> Self {
        self.documentation_url = Some(value.into());
        self
    }

    pub fn openapi_url(mut self, value: impl Into<String>) -> Self {
        self.openapi_url = Some(value.into());
        self
    }

    pub fn rate_limit(mut self, value: CheckStatusResponseRateLimit) -> Self {
        self.rate_limit = Some(value);
        self
    }

    pub fn status(mut self, value: impl Into<String>) -> Self {
        self.status = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`CheckStatusResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`api_version`](CheckStatusResponseBuilder::api_version)
    /// - [`authentication`](CheckStatusResponseBuilder::authentication)
    /// - [`developer_portal_url`](CheckStatusResponseBuilder::developer_portal_url)
    /// - [`documentation_url`](CheckStatusResponseBuilder::documentation_url)
    /// - [`openapi_url`](CheckStatusResponseBuilder::openapi_url)
    /// - [`rate_limit`](CheckStatusResponseBuilder::rate_limit)
    /// - [`status`](CheckStatusResponseBuilder::status)
    pub fn build(self) -> Result<CheckStatusResponse, BuildError> {
        Ok(CheckStatusResponse {
            api_version: self
                .api_version
                .ok_or_else(|| BuildError::missing_field("api_version"))?,
            authentication: self
                .authentication
                .ok_or_else(|| BuildError::missing_field("authentication"))?,
            developer_portal_url: self
                .developer_portal_url
                .ok_or_else(|| BuildError::missing_field("developer_portal_url"))?,
            documentation_url: self
                .documentation_url
                .ok_or_else(|| BuildError::missing_field("documentation_url"))?,
            openapi_url: self
                .openapi_url
                .ok_or_else(|| BuildError::missing_field("openapi_url"))?,
            rate_limit: self
                .rate_limit
                .ok_or_else(|| BuildError::missing_field("rate_limit"))?,
            status: self
                .status
                .ok_or_else(|| BuildError::missing_field("status"))?,
        })
    }
}
