pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct PatchApiV1AccountsAccountIdWebhookSubscriptionsIdResponse {
    /// Parent store ID
    #[serde(default)]
    pub account_id: i64,
    /// ISO 8601 creation timestamp
    #[serde(default)]
    pub created_at: String,
    /// Your own label for the subscription
    #[serde(default)]
    pub description: String,
    /// ISO 8601 time the subscription was disabled
    #[serde(default)]
    pub disabled_at: String,
    /// `disabled_by_user`, `failing` or `blocked_address`
    #[serde(default)]
    pub disabled_reason: String,
    /// Whether events are being delivered
    #[serde(default)]
    pub enabled: bool,
    /// The event, when there is exactly one (kept for older integrations)
    #[serde(default)]
    pub event: String,
    /// Events delivered to this URL, or `["*"]` for every event
    #[serde(default)]
    pub events: Vec<String>,
    /// Unique subscription ID
    #[serde(default)]
    pub id: i64,
    /// ISO 8601 time of the most recent delivery attempt
    #[serde(default)]
    pub last_delivery_at: String,
    /// Why the most recent attempt failed
    #[serde(default)]
    pub last_delivery_error: String,
    /// HTTP status your URL returned on the most recent attempt
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers")]
    pub last_delivery_status: f64,
    /// `envelope` (default) or `flat` (the bare data object, used by Zapier)
    #[serde(default)]
    pub payload_format: String,
    /// URL that receives the POST requests
    #[serde(default)]
    pub target_url: String,
    /// ISO 8601 last-update timestamp
    #[serde(default)]
    pub updated_at: String,
}

impl PatchApiV1AccountsAccountIdWebhookSubscriptionsIdResponse {
    pub fn builder() -> PatchApiV1AccountsAccountIdWebhookSubscriptionsIdResponseBuilder {
        <PatchApiV1AccountsAccountIdWebhookSubscriptionsIdResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PatchApiV1AccountsAccountIdWebhookSubscriptionsIdResponseBuilder {
    account_id: Option<i64>,
    created_at: Option<String>,
    description: Option<String>,
    disabled_at: Option<String>,
    disabled_reason: Option<String>,
    enabled: Option<bool>,
    event: Option<String>,
    events: Option<Vec<String>>,
    id: Option<i64>,
    last_delivery_at: Option<String>,
    last_delivery_error: Option<String>,
    last_delivery_status: Option<f64>,
    payload_format: Option<String>,
    target_url: Option<String>,
    updated_at: Option<String>,
}

impl PatchApiV1AccountsAccountIdWebhookSubscriptionsIdResponseBuilder {
    pub fn account_id(mut self, value: i64) -> Self {
        self.account_id = Some(value);
        self
    }

    pub fn created_at(mut self, value: impl Into<String>) -> Self {
        self.created_at = Some(value.into());
        self
    }

    pub fn description(mut self, value: impl Into<String>) -> Self {
        self.description = Some(value.into());
        self
    }

    pub fn disabled_at(mut self, value: impl Into<String>) -> Self {
        self.disabled_at = Some(value.into());
        self
    }

    pub fn disabled_reason(mut self, value: impl Into<String>) -> Self {
        self.disabled_reason = Some(value.into());
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

    pub fn id(mut self, value: i64) -> Self {
        self.id = Some(value);
        self
    }

    pub fn last_delivery_at(mut self, value: impl Into<String>) -> Self {
        self.last_delivery_at = Some(value.into());
        self
    }

    pub fn last_delivery_error(mut self, value: impl Into<String>) -> Self {
        self.last_delivery_error = Some(value.into());
        self
    }

    pub fn last_delivery_status(mut self, value: f64) -> Self {
        self.last_delivery_status = Some(value);
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

    pub fn updated_at(mut self, value: impl Into<String>) -> Self {
        self.updated_at = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`PatchApiV1AccountsAccountIdWebhookSubscriptionsIdResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`account_id`](PatchApiV1AccountsAccountIdWebhookSubscriptionsIdResponseBuilder::account_id)
    /// - [`created_at`](PatchApiV1AccountsAccountIdWebhookSubscriptionsIdResponseBuilder::created_at)
    /// - [`description`](PatchApiV1AccountsAccountIdWebhookSubscriptionsIdResponseBuilder::description)
    /// - [`disabled_at`](PatchApiV1AccountsAccountIdWebhookSubscriptionsIdResponseBuilder::disabled_at)
    /// - [`disabled_reason`](PatchApiV1AccountsAccountIdWebhookSubscriptionsIdResponseBuilder::disabled_reason)
    /// - [`enabled`](PatchApiV1AccountsAccountIdWebhookSubscriptionsIdResponseBuilder::enabled)
    /// - [`event`](PatchApiV1AccountsAccountIdWebhookSubscriptionsIdResponseBuilder::event)
    /// - [`events`](PatchApiV1AccountsAccountIdWebhookSubscriptionsIdResponseBuilder::events)
    /// - [`id`](PatchApiV1AccountsAccountIdWebhookSubscriptionsIdResponseBuilder::id)
    /// - [`last_delivery_at`](PatchApiV1AccountsAccountIdWebhookSubscriptionsIdResponseBuilder::last_delivery_at)
    /// - [`last_delivery_error`](PatchApiV1AccountsAccountIdWebhookSubscriptionsIdResponseBuilder::last_delivery_error)
    /// - [`last_delivery_status`](PatchApiV1AccountsAccountIdWebhookSubscriptionsIdResponseBuilder::last_delivery_status)
    /// - [`payload_format`](PatchApiV1AccountsAccountIdWebhookSubscriptionsIdResponseBuilder::payload_format)
    /// - [`target_url`](PatchApiV1AccountsAccountIdWebhookSubscriptionsIdResponseBuilder::target_url)
    /// - [`updated_at`](PatchApiV1AccountsAccountIdWebhookSubscriptionsIdResponseBuilder::updated_at)
    pub fn build(
        self,
    ) -> Result<PatchApiV1AccountsAccountIdWebhookSubscriptionsIdResponse, BuildError> {
        Ok(PatchApiV1AccountsAccountIdWebhookSubscriptionsIdResponse {
            account_id: self
                .account_id
                .ok_or_else(|| BuildError::missing_field("account_id"))?,
            created_at: self
                .created_at
                .ok_or_else(|| BuildError::missing_field("created_at"))?,
            description: self
                .description
                .ok_or_else(|| BuildError::missing_field("description"))?,
            disabled_at: self
                .disabled_at
                .ok_or_else(|| BuildError::missing_field("disabled_at"))?,
            disabled_reason: self
                .disabled_reason
                .ok_or_else(|| BuildError::missing_field("disabled_reason"))?,
            enabled: self
                .enabled
                .ok_or_else(|| BuildError::missing_field("enabled"))?,
            event: self
                .event
                .ok_or_else(|| BuildError::missing_field("event"))?,
            events: self
                .events
                .ok_or_else(|| BuildError::missing_field("events"))?,
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            last_delivery_at: self
                .last_delivery_at
                .ok_or_else(|| BuildError::missing_field("last_delivery_at"))?,
            last_delivery_error: self
                .last_delivery_error
                .ok_or_else(|| BuildError::missing_field("last_delivery_error"))?,
            last_delivery_status: self
                .last_delivery_status
                .ok_or_else(|| BuildError::missing_field("last_delivery_status"))?,
            payload_format: self
                .payload_format
                .ok_or_else(|| BuildError::missing_field("payload_format"))?,
            target_url: self
                .target_url
                .ok_or_else(|| BuildError::missing_field("target_url"))?,
            updated_at: self
                .updated_at
                .ok_or_else(|| BuildError::missing_field("updated_at"))?,
        })
    }
}
