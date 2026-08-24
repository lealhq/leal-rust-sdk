use crate::api::*;
use crate::{ApiError, ClientConfig, HttpClient, QueryBuilder, RequestOptions};
use reqwest::Method;

pub struct CardsClient {
    pub http_client: HttpClient,
}

impl CardsClient {
    pub fn new(config: ClientConfig) -> Result<Self, ApiError> {
        Ok(Self {
            http_client: HttpClient::new(config.clone())?,
        })
    }

    /// Returns loyalty card templates for the specified store. By default, only
    /// active (unarchived) cards are returned. Use the `scope` parameter to include
    /// archived cards.
    ///
    /// # Arguments
    ///
    /// * `account_id` - Parent store ID
    /// * `scope` - Filter cards by archive status. Default: active only.
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
    ///         .cards
    ///         .list(
    ///             1,
    ///             &CardsListQueryRequest {
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
        request: &CardsListQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<Vec<ListCardsResponseItem>, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                &format!("api/v1/accounts/{}/cards", account_id),
                None,
                QueryBuilder::new()
                    .string("scope", request.scope.clone())
                    .build(),
                options,
            )
            .await
    }

    /// Creates a new loyalty stamp card template for the store. The card defines the
    /// visual design (colours, icon, strip) and program rules (stamps required,
    /// initial stamps).
    ///
    /// # Arguments
    ///
    /// * `account_id` - Parent store ID
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
    ///         .cards
    ///         .create(
    ///             1,
    ///             &CreateCardsRequest {
    ///                 card: CreateCardsRequestCard {
    ///                     name: "name".to_string(),
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
        request: &CreateCardsRequest,
        options: Option<RequestOptions>,
    ) -> Result<CreateCardsResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                &format!("api/v1/accounts/{}/cards", account_id),
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Returns a single loyalty card template by ID, including reward and customer card counts.
    ///
    /// # Arguments
    ///
    /// * `account_id` - Parent store ID
    /// * `id` - Card ID
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
    ///     client.cards.get(1, 1, None).await;
    /// }
    /// ```
    pub async fn get(
        &self,
        account_id: i64,
        id: i64,
        options: Option<RequestOptions>,
    ) -> Result<GetCardsResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                &format!("api/v1/accounts/{}/cards/{}", account_id, id),
                None,
                None,
                options,
            )
            .await
    }

    /// Updates an existing loyalty card template. Only the provided attributes are changed.
    ///
    /// # Arguments
    ///
    /// * `account_id` - Parent store ID
    /// * `id` - Card ID
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
    ///         .cards
    ///         .update(
    ///             1,
    ///             1,
    ///             &UpdateCardsRequest {
    ///                 card: UpdateCardsRequestCard {
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
        request: &UpdateCardsRequest,
        options: Option<RequestOptions>,
    ) -> Result<UpdateCardsResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::PATCH,
                &format!("api/v1/accounts/{}/cards/{}", account_id, id),
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }
}
