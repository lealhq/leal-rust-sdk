pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostApiV1AccountsAccountIdWebhookSubscriptionsRequest {
    /// Your own label, up to 255 characters
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// Create the subscription disabled by passing false (defaults to true)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
    /// A single event to subscribe to. Same as `events` with one entry
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event: Option<String>,
    /// Events to subscribe to, or `["*"]` for every event. Required unless `event` is given
    #[serde(skip_serializing_if = "Option::is_none")]
    pub events: Option<Vec<String>>,
    /// `envelope` (default) or `flat`. `flat` sends the bare data object and cannot be combined with `*`
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payload_format: Option<String>,
    /// Public https URL that will receive the POST requests
    #[serde(default)]
    pub target_url: String,
}

impl PostApiV1AccountsAccountIdWebhookSubscriptionsRequest {
    pub fn builder() -> PostApiV1AccountsAccountIdWebhookSubscriptionsRequestBuilder {
        <PostApiV1AccountsAccountIdWebhookSubscriptionsRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostApiV1AccountsAccountIdWebhookSubscriptionsRequestBuilder {
    description: Option<String>,
    enabled: Option<bool>,
    event: Option<String>,
    events: Option<Vec<String>>,
    payload_format: Option<String>,
    target_url: Option<String>,
}

impl PostApiV1AccountsAccountIdWebhookSubscriptionsRequestBuilder {
    pub fn description(mut self, value: impl Into<String>) -> Self {
        self.description = Some(value.into());
        self
    }

    pub fn enabled(mut self, value: bool) -> Self {
        self.enabled = Some(value);
        self
    }

    pub fn event(mut self, value: impl Into<String>) -> Self {
        self.event = Some(value.into());
        self
    }

    pub fn events(mut self, value: Vec<String>) -> Self {
        self.events = Some(value);
        self
    }

    pub fn payload_format(mut self, value: impl Into<String>) -> Self {
        self.payload_format = Some(value.into());
        self
    }

    pub fn target_url(mut self, value: impl Into<String>) -> Self {
        self.target_url = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`PostApiV1AccountsAccountIdWebhookSubscriptionsRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`target_url`](PostApiV1AccountsAccountIdWebhookSubscriptionsRequestBuilder::target_url)
    pub fn build(
        self,
    ) -> Result<PostApiV1AccountsAccountIdWebhookSubscriptionsRequest, BuildError> {
        Ok(PostApiV1AccountsAccountIdWebhookSubscriptionsRequest {
            description: self.description,
            enabled: self.enabled,
            event: self.event,
            events: self.events,
            payload_format: self.payload_format,
            target_url: self
                .target_url
                .ok_or_else(|| BuildError::missing_field("target_url"))?,
        })
    }
}
