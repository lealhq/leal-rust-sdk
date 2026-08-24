use crate::api::*;
use crate::{ApiError, ClientConfig, HttpClient, RequestOptions};
use reqwest::Method;

pub struct CustomerCardsClient {
    pub http_client: HttpClient,
}

impl CustomerCardsClient {
    pub fn new(config: ClientConfig) -> Result<Self, ApiError> {
        Ok(Self {
            http_client: HttpClient::new(config.clone())?,
        })
    }

    /// Returns all loyalty cards enrolled for a specific customer, including stamp progress,
    /// status, wallet pass installation state, and wallet pass URLs (`apple_wallet_url` and
    /// `google_wallet_url`) that you can use to let customers add their loyalty card to
    /// Apple Wallet or Google Wallet from your own app or website.
    ///
    /// # Arguments
    ///
    /// * `account_id` - Store (account) ID
    /// * `customer_id` - Customer ID
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
    ///     client.customer_cards.list(1, 1, None).await;
    /// }
    /// ```
    pub async fn list(
        &self,
        account_id: i64,
        customer_id: i64,
        options: Option<RequestOptions>,
    ) -> Result<Vec<ListCustomerCardsResponseItem>, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                &format!(
                    "api/v1/accounts/{}/customers/{}/customer_cards",
                    account_id, customer_id
                ),
                None,
                None,
                options,
            )
            .await
    }

    /// Returns detailed information about a specific customer card, including stamp progress,
    /// a list of rewards the customer has earned enough stamps to redeem, and wallet pass URLs
    /// (`apple_wallet_url` and `google_wallet_url`) for adding the card to Apple Wallet or
    /// Google Wallet.
    ///
    /// # Arguments
    ///
    /// * `account_id` - Store (account) ID
    /// * `customer_id` - Customer ID
    /// * `id` - Customer card ID
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
    ///     client.customer_cards.get(1, 1, 1, None).await;
    /// }
    /// ```
    pub async fn get(
        &self,
        account_id: i64,
        customer_id: i64,
        id: i64,
        options: Option<RequestOptions>,
    ) -> Result<GetCustomerCardsResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                &format!(
                    "api/v1/accounts/{}/customers/{}/customer_cards/{}",
                    account_id, customer_id, id
                ),
                None,
                None,
                options,
            )
            .await
    }

    /// Redeems a reward for a customer, deducting the required stamps from their card.
    /// The customer must have enough stamps on this card to cover the reward's cost.
    /// Triggers wallet pass updates and push notifications.
    ///
    /// # Arguments
    ///
    /// * `account_id` - Store (account) ID
    /// * `customer_id` - Customer ID
    /// * `id` - Customer card ID
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
    ///         .customer_cards
    ///         .redeem(1, 1, 1, &RedeemCustomerCardsRequest { reward_id: 1 }, None)
    ///         .await;
    /// }
    /// ```
    pub async fn redeem(
        &self,
        account_id: i64,
        customer_id: i64,
        id: i64,
        request: &RedeemCustomerCardsRequest,
        options: Option<RequestOptions>,
    ) -> Result<RedeemCustomerCardsResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                &format!(
                    "api/v1/accounts/{}/customers/{}/customer_cards/{}/redeem",
                    account_id, customer_id, id
                ),
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Adds stamps to a customer's loyalty card. Triggers ledger entries, wallet pass updates,
    /// and push notifications. Pass `skip_notifications` to stamp silently.
    ///
    /// # Arguments
    ///
    /// * `account_id` - Store (account) ID
    /// * `customer_id` - Customer ID
    /// * `id` - Customer card ID
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
    ///         .customer_cards
    ///         .stamp(
    ///             1,
    ///             1,
    ///             1,
    ///             &StampCustomerCardsRequest {
    ///                 stamps: 1,
    ///                 skip_notifications: None,
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn stamp(
        &self,
        account_id: i64,
        customer_id: i64,
        id: i64,
        request: &StampCustomerCardsRequest,
        options: Option<RequestOptions>,
    ) -> Result<StampCustomerCardsResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                &format!(
                    "api/v1/accounts/{}/customers/{}/customer_cards/{}/stamp",
                    account_id, customer_id, id
                ),
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }
}
