use crate::api::*;
use crate::{ApiError, ClientConfig, HttpClient, RequestOptions};
use reqwest::Method;

pub struct StoresClient {
    pub http_client: HttpClient,
}

impl StoresClient {
    pub fn new(config: ClientConfig) -> Result<Self, ApiError> {
        Ok(Self {
            http_client: HttpClient::new(config.clone())?,
        })
    }

    /// Returns every store the authenticated user has access to, including summary counts for locations, cards, customers, and posters.
    ///
    /// # Arguments
    ///
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
    ///     client.stores.list(None).await;
    /// }
    /// ```
    pub async fn list(
        &self,
        options: Option<RequestOptions>,
    ) -> Result<Vec<ListStoresResponseItem>, ApiError> {
        self.http_client
            .execute_request(Method::GET, "api/v1/accounts", None, None, options)
            .await
    }

    /// Returns detailed information for a single store, including summary counts for its associated resources.
    ///
    /// # Arguments
    ///
    /// * `id` - Store ID
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
    ///     client.stores.get(1, None).await;
    /// }
    /// ```
    pub async fn get(
        &self,
        id: i64,
        options: Option<RequestOptions>,
    ) -> Result<GetStoresResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                &format!("api/v1/accounts/{}", id),
                None,
                None,
                options,
            )
            .await
    }

    /// Updates the store's name or store_name. Use `store_name` for the public-facing name displayed to customers.
    ///
    /// # Arguments
    ///
    /// * `id` - Store ID
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
    ///         .stores
    ///         .update(
    ///             1,
    ///             &UpdateStoresRequest {
    ///                 account: UpdateStoresRequestAccount {
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
        id: i64,
        request: &UpdateStoresRequest,
        options: Option<RequestOptions>,
    ) -> Result<UpdateStoresResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::PATCH,
                &format!("api/v1/accounts/{}", id),
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }
}
