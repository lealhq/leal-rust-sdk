pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PatchApiV1AccountsAccountIdWebhookSubscriptionsIdRequest {
    /// Your own label, up to 255 characters
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// false to pause deliveries, true to resume them
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
    /// A single event. Same as `events` with one entry
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event: Option<String>,
    /// Replaces the list of events, or `["*"]` for every event
    #[serde(skip_serializing_if = "Option::is_none")]
    pub events: Option<Vec<String>>,
    /// `envelope` or `flat`
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payload_format: Option<String>,
    /// Public https URL that will receive the POST requests
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_url: Option<String>,
}

impl PatchApiV1AccountsAccountIdWebhookSubscriptionsIdRequest {
    pub fn builder() -> PatchApiV1AccountsAccountIdWebhookSubscriptionsIdRequestBuilder {
        <PatchApiV1AccountsAccountIdWebhookSubscriptionsIdRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PatchApiV1AccountsAccountIdWebhookSubscriptionsIdRequestBuilder {
    description: Option<String>,
    enabled: Option<bool>,
    event: Option<String>,
    events: Option<Vec<String>>,
    payload_format: Option<String>,
    target_url: Option<String>,
}

impl PatchApiV1AccountsAccountIdWebhookSubscriptionsIdRequestBuilder {
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

    /// Consumes the builder and constructs a [`PatchApiV1AccountsAccountIdWebhookSubscriptionsIdRequest`].
    pub fn build(
        self,
    ) -> Result<PatchApiV1AccountsAccountIdWebhookSubscriptionsIdRequest, BuildError> {
        Ok(PatchApiV1AccountsAccountIdWebhookSubscriptionsIdRequest {
            description: self.description,
            enabled: self.enabled,
            event: self.event,
            events: self.events,
            payload_format: self.payload_format,
            target_url: self.target_url,
        })
    }
}
