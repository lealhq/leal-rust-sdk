use crate::api::*;
use crate::{ApiError, ClientConfig, HttpClient, QueryBuilder, RequestOptions};
use reqwest::Method;

pub struct PostersClient {
    pub http_client: HttpClient,
}

impl PostersClient {
    pub fn new(config: ClientConfig) -> Result<Self, ApiError> {
        Ok(Self {
            http_client: HttpClient::new(config.clone())?,
        })
    }

    /// Returns all posters for the store. Optionally filter by card or active status.
    ///
    /// # Arguments
    ///
    /// * `account_id` - Store (account) ID
    /// * `card_id` - Filter posters belonging to a specific card
    /// * `active` - When present, return only active posters
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
    ///         .posters
    ///         .list(
    ///             1,
    ///             &PostersListQueryRequest {
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
        request: &PostersListQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<Vec<ListPostersResponseItem>, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                &format!("api/v1/accounts/{}/posters", account_id),
                None,
                QueryBuilder::new()
                    .int("card_id", request.card_id.clone())
                    .string("active", request.active.clone())
                    .build(),
                options,
            )
            .await
    }

    /// Creates a new printable QR code poster for customer signup. The poster will automatically
    /// generate a unique public signup URL and QR code. The `card_id` is required on create to
    /// associate the poster with a loyalty card.
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
    ///         .posters
    ///         .create(
    ///             1,
    ///             &CreatePostersRequest {
    ///                 poster: CreatePostersRequestPoster {
    ///                     card_id: 1,
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
        request: &CreatePostersRequest,
        options: Option<RequestOptions>,
    ) -> Result<CreatePostersResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                &format!("api/v1/accounts/{}/posters", account_id),
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Returns a single poster by ID, including generated signup and display URLs.
    ///
    /// # Arguments
    ///
    /// * `account_id` - Store (account) ID
    /// * `id` - Poster ID
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
    ///     client.posters.get(1, 1, None).await;
    /// }
    /// ```
    pub async fn get(
        &self,
        account_id: i64,
        id: i64,
        options: Option<RequestOptions>,
    ) -> Result<GetPostersResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                &format!("api/v1/accounts/{}/posters/{}", account_id, id),
                None,
                None,
                options,
            )
            .await
    }

    /// Permanently deletes a poster. The public signup URL will stop working.
    ///
    /// # Arguments
    ///
    /// * `account_id` - Store (account) ID
    /// * `id` - Poster ID
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
    ///     client.posters.delete(1, 1, None).await;
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
                &format!("api/v1/accounts/{}/posters/{}", account_id, id),
                None,
                None,
                options,
            )
            .await
    }

    /// Updates an existing poster. The `card_id` cannot be changed after creation.
    ///
    /// # Arguments
    ///
    /// * `account_id` - Store (account) ID
    /// * `id` - Poster ID
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
    ///         .posters
    ///         .update(
    ///             1,
    ///             1,
    ///             &UpdatePostersRequest {
    ///                 poster: UpdatePostersRequestPoster {
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
        request: &UpdatePostersRequest,
        options: Option<RequestOptions>,
    ) -> Result<UpdatePostersResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::PATCH,
                &format!("api/v1/accounts/{}/posters/{}", account_id, id),
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }
}
