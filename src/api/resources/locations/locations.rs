use crate::api::*;
use crate::{ApiError, ClientConfig, HttpClient, RequestOptions};
use reqwest::Method;

pub struct LocationsClient {
    pub http_client: HttpClient,
}

impl LocationsClient {
    pub fn new(config: ClientConfig) -> Result<Self, ApiError> {
        Ok(Self {
            http_client: HttpClient::new(config.clone())?,
        })
    }

    /// Returns every physical location belonging to the specified store.
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
    ///     client.locations.list(1, None).await;
    /// }
    /// ```
    pub async fn list(
        &self,
        account_id: i64,
        options: Option<RequestOptions>,
    ) -> Result<Vec<ListLocationsResponseItem>, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                &format!("api/v1/accounts/{}/locations", account_id),
                None,
                None,
                options,
            )
            .await
    }

    /// Creates a new physical location for the store. The provided address is
    /// automatically geocoded to latitude and longitude coordinates in the background.
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
    ///         .locations
    ///         .create(
    ///             1,
    ///             &CreateLocationsRequest {
    ///                 location: CreateLocationsRequestLocation {
    ///                     address: "address".to_string(),
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
        request: &CreateLocationsRequest,
        options: Option<RequestOptions>,
    ) -> Result<CreateLocationsResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                &format!("api/v1/accounts/{}/locations", account_id),
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Returns a single location by ID.
    ///
    /// # Arguments
    ///
    /// * `account_id` - Parent store ID
    /// * `id` - Location ID
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
    ///     client.locations.get(1, 1, None).await;
    /// }
    /// ```
    pub async fn get(
        &self,
        account_id: i64,
        id: i64,
        options: Option<RequestOptions>,
    ) -> Result<GetLocationsResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                &format!("api/v1/accounts/{}/locations/{}", account_id, id),
                None,
                None,
                options,
            )
            .await
    }

    /// Permanently deletes a location. This action cannot be undone.
    ///
    /// # Arguments
    ///
    /// * `account_id` - Parent store ID
    /// * `id` - Location ID
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
    ///     client.locations.delete(1, 1, None).await;
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
                &format!("api/v1/accounts/{}/locations/{}", account_id, id),
                None,
                None,
                options,
            )
            .await
    }

    /// Updates an existing location. If the address is changed, it will be re-geocoded automatically.
    ///
    /// # Arguments
    ///
    /// * `account_id` - Parent store ID
    /// * `id` - Location ID
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
    ///         .locations
    ///         .update(
    ///             1,
    ///             1,
    ///             &UpdateLocationsRequest {
    ///                 location: UpdateLocationsRequestLocation {
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
        request: &UpdateLocationsRequest,
        options: Option<RequestOptions>,
    ) -> Result<UpdateLocationsResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::PATCH,
                &format!("api/v1/accounts/{}/locations/{}", account_id, id),
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }
}
