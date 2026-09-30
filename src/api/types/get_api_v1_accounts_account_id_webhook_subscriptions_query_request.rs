pub use crate::prelude::*;

/// Query parameters for get_api_v1_accounts_account_id_webhook_subscriptions
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct GetApiV1AccountsAccountIdWebhookSubscriptionsQueryRequest {
    /// Only return subscriptions that list this event (or `*`)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event: Option<String>,
}

impl GetApiV1AccountsAccountIdWebhookSubscriptionsQueryRequest {
    pub fn builder() -> GetApiV1AccountsAccountIdWebhookSubscriptionsQueryRequestBuilder {
        <GetApiV1AccountsAccountIdWebhookSubscriptionsQueryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct GetApiV1AccountsAccountIdWebhookSubscriptionsQueryRequestBuilder {
    event: Option<String>,
}

impl GetApiV1AccountsAccountIdWebhookSubscriptionsQueryRequestBuilder {
    pub fn event(mut self, value: impl Into<String>) -> Self {
        self.event = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`GetApiV1AccountsAccountIdWebhookSubscriptionsQueryRequest`].
    pub fn build(
        self,
    ) -> Result<GetApiV1AccountsAccountIdWebhookSubscriptionsQueryRequest, BuildError> {
        Ok(GetApiV1AccountsAccountIdWebhookSubscriptionsQueryRequest { event: self.event })
    }
}
