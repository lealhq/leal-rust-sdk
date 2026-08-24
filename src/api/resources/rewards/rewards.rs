use crate::api::*;
use crate::{ApiError, ClientConfig, HttpClient, QueryBuilder, RequestOptions};
use reqwest::Method;

pub struct RewardsClient {
    pub http_client: HttpClient,
}

impl RewardsClient {
    pub fn new(config: ClientConfig) -> Result<Self, ApiError> {
        Ok(Self {
            http_client: HttpClient::new(config.clone())?,
        })
    }

    /// Returns all rewards for the store. Optionally filter by card or active status.
    ///
    /// # Arguments
    ///
    /// * `account_id` - Store (account) ID
    /// * `card_id` - Filter rewards belonging to a specific card
    /// * `active` - When present, return only active rewards
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
    ///         .rewards
    ///         .list(
    ///             1,
    ///             &RewardsListQueryRequest {
    ///                 ..Default::default()
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn list(
        &self,
        account_id: i64,
        request: &RewardsListQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<Vec<ListRewardsResponseItem>, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                &format!("api/v1/accounts/{}/rewards", account_id),
                None,
                QueryBuilder::new()
                    .int("card_id", request.card_id.clone())
                    .string("active", request.active.clone())
                    .build(),
                options,
            )
            .await
    }

    /// Creates a new reward for a loyalty card. The card must belong to the same store.
    /// The `card_id` is required on create but cannot be changed afterwards.
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
    ///         .rewards
    ///         .create(
    ///             1,
    ///             &CreateRewardsRequest {
    ///                 reward: CreateRewardsRequestReward {
    ///                     card_id: 1,
    ///                     name: "name".to_string(),
    ///                     stamps_required: 1,
    ///                     ..Default::default()
    ///                 },
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn create(
        &self,
        account_id: i64,
        request: &CreateRewardsRequest,
        options: Option<RequestOptions>,
    ) -> Result<CreateRewardsResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                &format!("api/v1/accounts/{}/rewards", account_id),
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Returns a single reward by ID.
    ///
    /// # Arguments
    ///
    /// * `account_id` - Store (account) ID
    /// * `id` - Reward ID
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
    ///     client.rewards.get(1, 1, None).await;
    /// }
    /// ```
    pub async fn get(
        &self,
        account_id: i64,
        id: i64,
        options: Option<RequestOptions>,
    ) -> Result<GetRewardsResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                &format!("api/v1/accounts/{}/rewards/{}", account_id, id),
                None,
                None,
                options,
            )
            .await
    }

    /// Permanently deletes a reward. This cannot be undone.
    ///
    /// # Arguments
    ///
    /// * `account_id` - Store (account) ID
    /// * `id` - Reward ID
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
    ///     client.rewards.delete(1, 1, None).await;
    /// }
    /// ```
    pub async fn delete(
        &self,
        account_id: i64,
        id: i64,
        options: Option<RequestOptions>,
    ) -> Result<(), ApiError> {
        self.http_client
            .execute_request(
                Method::DELETE,
                &format!("api/v1/accounts/{}/rewards/{}", account_id, id),
                None,
                None,
                options,
            )
            .await
    }

    /// Updates an existing reward. The `card_id` cannot be changed after creation.
    ///
    /// # Arguments
    ///
    /// * `account_id` - Store (account) ID
    /// * `id` - Reward ID
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
    ///         .rewards
    ///         .update(
    ///             1,
    ///             1,
    ///             &UpdateRewardsRequest {
    ///                 reward: UpdateRewardsRequestReward {
    ///                     ..Default::default()
    ///                 },
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn update(
        &self,
        account_id: i64,
        id: i64,
        request: &UpdateRewardsRequest,
        options: Option<RequestOptions>,
    ) -> Result<UpdateRewardsResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::PATCH,
                &format!("api/v1/accounts/{}/rewards/{}", account_id, id),
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }
}
