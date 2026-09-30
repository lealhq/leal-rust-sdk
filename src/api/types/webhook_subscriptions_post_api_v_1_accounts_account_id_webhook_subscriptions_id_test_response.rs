pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct PostApiV1AccountsAccountIdWebhookSubscriptionsIdTestResponse {
    /// True when your URL responded with a 2xx status
    #[serde(default)]
    pub delivered: bool,
    /// Why the delivery failed
    #[serde(default)]
    pub error: String,
    /// The `webhook-id` of the test event
    #[serde(default)]
    pub event_id: String,
    /// HTTP status your URL returned
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers")]
    pub status: f64,
}

impl PostApiV1AccountsAccountIdWebhookSubscriptionsIdTestResponse {
    pub fn builder() -> PostApiV1AccountsAccountIdWebhookSubscriptionsIdTestResponseBuilder {
        <PostApiV1AccountsAccountIdWebhookSubscriptionsIdTestResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostApiV1AccountsAccountIdWebhookSubscriptionsIdTestResponseBuilder {
    delivered: Option<bool>,
    error: Option<String>,
    event_id: Option<String>,
    status: Option<f64>,
}

impl PostApiV1AccountsAccountIdWebhookSubscriptionsIdTestResponseBuilder {
    pub fn delivered(mut self, value: bool) -> Self {
        self.delivered = Some(value);
        self
    }

    pub fn error(mut self, value: impl Into<String>) -> Self {
        self.error = Some(value.into());
        self
    }

    pub fn event_id(mut self, value: impl Into<String>) -> Self {
        self.event_id = Some(value.into());
        self
    }

    pub fn status(mut self, value: f64) -> Self {
        self.status = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PostApiV1AccountsAccountIdWebhookSubscriptionsIdTestResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`delivered`](PostApiV1AccountsAccountIdWebhookSubscriptionsIdTestResponseBuilder::delivered)
    /// - [`error`](PostApiV1AccountsAccountIdWebhookSubscriptionsIdTestResponseBuilder::error)
    /// - [`event_id`](PostApiV1AccountsAccountIdWebhookSubscriptionsIdTestResponseBuilder::event_id)
    /// - [`status`](PostApiV1AccountsAccountIdWebhookSubscriptionsIdTestResponseBuilder::status)
    pub fn build(
        self,
    ) -> Result<PostApiV1AccountsAccountIdWebhookSubscriptionsIdTestResponse, BuildError> {
        Ok(
            PostApiV1AccountsAccountIdWebhookSubscriptionsIdTestResponse {
                delivered: self
                    .delivered
                    .ok_or_else(|| BuildError::missing_field("delivered"))?,
                error: self
                    .error
                    .ok_or_else(|| BuildError::missing_field("error"))?,
                event_id: self
                    .event_id
                    .ok_or_else(|| BuildError::missing_field("event_id"))?,
                status: self
                    .status
                    .ok_or_else(|| BuildError::missing_field("status"))?,
            },
        )
    }
}
