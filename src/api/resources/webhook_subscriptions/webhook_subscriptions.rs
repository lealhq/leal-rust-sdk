use crate::api::*;
use crate::{ApiError, ClientConfig, HttpClient, QueryBuilder, RequestOptions};
use reqwest::Method;

pub struct WebhookSubscriptionsClient {
    pub http_client: HttpClient,
}

impl WebhookSubscriptionsClient {
    pub fn new(config: ClientConfig) -> Result<Self, ApiError> {
        Ok(Self {
            http_client: HttpClient::new(config.clone())?,
        })
    }

    /// Returns every webhook subscription for the store, oldest first. Signing secrets are not included; fetch a single subscription to read its secret.
    ///
    /// # Arguments
    ///
    /// * `account_id` - Store (account) ID
    /// * `event` - Only return subscriptions that list this event (or `*`)
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use leal::prelude::*;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let config = ClientConfig {
    ///         token: Some("<token>".to_string()),
    ///         ..Default::default()
    ///     };
    ///     let client = LealClient::new(config).expect("Failed to build client");
    ///     client
    ///         .webhook_subscriptions
    ///         .get_api_v1accounts_account_id_webhook_subscriptions(
    ///             1,
    ///             &GetAPIV1AccountsAccountIDWebhookSubscriptionsQueryRequest {
    ///                 ..Default::default()
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn get_api_v1accounts_account_id_webhook_subscriptions(
        &self,
        account_id: i64,
        request: &GetApiV1AccountsAccountIdWebhookSubscriptionsQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<Vec<GetApiV1AccountsAccountIdWebhookSubscriptionsResponseItem>, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                &format!("api/v1/accounts/{}/webhook_subscriptions", account_id),
                None,
                QueryBuilder::new()
                    .string("event", request.event.clone())
                    .build(),
                options,
            )
            .await
    }

    /// Subscribes a URL to one or more events. The response includes the signing `secret`; store it to
    /// verify deliveries. The URL must be publicly reachable over https.
    ///
    /// Events: `customer.created`, `customer.updated`, `customer_card.created`, `stamp.earned`, `stamp.removed`, `reward.unlocked`, `reward.redeemed`, or `*` for all of them.
    ///
    /// # Arguments
    ///
    /// * `account_id` - Store (account) ID
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use leal::prelude::*;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let config = ClientConfig {
    ///         token: Some("<token>".to_string()),
    ///         ..Default::default()
    ///     };
    ///     let client = LealClient::new(config).expect("Failed to build client");
    ///     client
    ///         .webhook_subscriptions
    ///         .post_api_v1accounts_account_id_webhook_subscriptions(
    ///             1,
    ///             &PostAPIV1AccountsAccountIDWebhookSubscriptionsRequest {
    ///                 target_url: "target_url".to_string(),
    ///                 description: None,
    ///                 enabled: None,
    ///                 event: None,
    ///                 events: None,
    ///                 payload_format: None,
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn post_api_v1accounts_account_id_webhook_subscriptions(
        &self,
        account_id: i64,
        request: &PostApiV1AccountsAccountIdWebhookSubscriptionsRequest,
        options: Option<RequestOptions>,
    ) -> Result<PostApiV1AccountsAccountIdWebhookSubscriptionsResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                &format!("api/v1/accounts/{}/webhook_subscriptions", account_id),
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Returns a single subscription, including its signing secret and the result of the most recent delivery.
    ///
    /// # Arguments
    ///
    /// * `account_id` - Store (account) ID
    /// * `id` - Webhook subscription ID
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use leal::prelude::*;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let config = ClientConfig {
    ///         token: Some("<token>".to_string()),
    ///         ..Default::default()
    ///     };
    ///     let client = LealClient::new(config).expect("Failed to build client");
    ///     client
    ///         .webhook_subscriptions
    ///         .get_api_v1accounts_account_id_webhook_subscriptions_id(1, 1, None)
    ///         .await;
    /// }
    /// ```
    pub async fn get_api_v1accounts_account_id_webhook_subscriptions_id(
        &self,
        account_id: i64,
        id: i64,
        options: Option<RequestOptions>,
    ) -> Result<GetApiV1AccountsAccountIdWebhookSubscriptionsIdResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                &format!(
                    "api/v1/accounts/{}/webhook_subscriptions/{}",
                    account_id, id
                ),
                None,
                None,
                options,
            )
            .await
    }

    /// Stops deliveries and deletes the subscription. This cannot be undone.
    ///
    /// # Arguments
    ///
    /// * `account_id` - Store (account) ID
    /// * `id` - Webhook subscription ID
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// Empty response
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use leal::prelude::*;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let config = ClientConfig {
    ///         token: Some("<token>".to_string()),
    ///         ..Default::default()
    ///     };
    ///     let client = LealClient::new(config).expect("Failed to build client");
    ///     client
    ///         .webhook_subscriptions
    ///         .delete_api_v1accounts_account_id_webhook_subscriptions_id(1, 1, None)
    ///         .await;
    /// }
    /// ```
    pub async fn delete_api_v1accounts_account_id_webhook_subscriptions_id(
        &self,
        account_id: i64,
        id: i64,
        options: Option<RequestOptions>,
    ) -> Result<(), ApiError> {
        self.http_client
            .execute_request(
                Method::DELETE,
                &format!(
                    "api/v1/accounts/{}/webhook_subscriptions/{}",
                    account_id, id
                ),
                None,
                None,
                options,
            )
            .await
    }

    /// Changes the URL, events, label or payload format, or turns the subscription off and on. Re-enabling a subscription that was disabled for failing clears its failure state.
    ///
    /// # Arguments
    ///
    /// * `account_id` - Store (account) ID
    /// * `id` - Webhook subscription ID
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use leal::prelude::*;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let config = ClientConfig {
    ///         token: Some("<token>".to_string()),
    ///         ..Default::default()
    ///     };
    ///     let client = LealClient::new(config).expect("Failed to build client");
    ///     client
    ///         .webhook_subscriptions
    ///         .patch_api_v1accounts_account_id_webhook_subscriptions_id(
    ///             1,
    ///             1,
    ///             &PatchAPIV1AccountsAccountIDWebhookSubscriptionsIDRequest {
    ///                 ..Default::default()
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn patch_api_v1accounts_account_id_webhook_subscriptions_id(
        &self,
        account_id: i64,
        id: i64,
        request: &PatchApiV1AccountsAccountIdWebhookSubscriptionsIdRequest,
        options: Option<RequestOptions>,
    ) -> Result<PatchApiV1AccountsAccountIdWebhookSubscriptionsIdResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::PATCH,
                &format!(
                    "api/v1/accounts/{}/webhook_subscriptions/{}",
                    account_id, id
                ),
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Replaces the subscription's signing secret. Deliveries are signed with the new secret straight away, so update your receiver at the same time.
    ///
    /// # Arguments
    ///
    /// * `account_id` - Store (account) ID
    /// * `id` - Webhook subscription ID
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use leal::prelude::*;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let config = ClientConfig {
    ///         token: Some("<token>".to_string()),
    ///         ..Default::default()
    ///     };
    ///     let client = LealClient::new(config).expect("Failed to build client");
    ///     client
    ///         .webhook_subscriptions
    ///         .post_api_v1accounts_account_id_webhook_subscriptions_id_rotate_secret(1, 1, None)
    ///         .await;
    /// }
    /// ```
    pub async fn post_api_v1accounts_account_id_webhook_subscriptions_id_rotate_secret(
        &self,
        account_id: i64,
        id: i64,
        options: Option<RequestOptions>,
    ) -> Result<PostApiV1AccountsAccountIdWebhookSubscriptionsIdRotateSecretResponse, ApiError>
    {
        self.http_client
            .execute_request(
                Method::POST,
                &format!(
                    "api/v1/accounts/{}/webhook_subscriptions/{}/rotate_secret",
                    account_id, id
                ),
                None,
                None,
                options,
            )
            .await
    }

    /// Immediately sends a signed `webhook.test` event to the subscription's URL and reports what
    /// happened, so you can check your endpoint and signature verification without waiting for real
    /// activity. Test events are not retried and do not count towards disabling the subscription.
    ///
    /// # Arguments
    ///
    /// * `account_id` - Store (account) ID
    /// * `id` - Webhook subscription ID
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use leal::prelude::*;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let config = ClientConfig {
    ///         token: Some("<token>".to_string()),
    ///         ..Default::default()
    ///     };
    ///     let client = LealClient::new(config).expect("Failed to build client");
    ///     client
    ///         .webhook_subscriptions
    ///         .post_api_v1accounts_account_id_webhook_subscriptions_id_test(1, 1, None)
    ///         .await;
    /// }
    /// ```
    pub async fn post_api_v1accounts_account_id_webhook_subscriptions_id_test(
        &self,
        account_id: i64,
        id: i64,
        options: Option<RequestOptions>,
    ) -> Result<PostApiV1AccountsAccountIdWebhookSubscriptionsIdTestResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                &format!(
                    "api/v1/accounts/{}/webhook_subscriptions/{}/test",
                    account_id, id
                ),
                None,
                None,
                options,
            )
            .await
    }
}
