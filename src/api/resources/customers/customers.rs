use crate::api::*;
use crate::{ApiError, ClientConfig, HttpClient, QueryBuilder, RequestOptions};
use reqwest::Method;

pub struct CustomersClient {
    pub http_client: HttpClient,
}

impl CustomersClient {
    pub fn new(config: ClientConfig) -> Result<Self, ApiError> {
        Ok(Self {
            http_client: HttpClient::new(config.clone())?,
        })
    }

    /// Returns a paginated list of customers for the store. Use the `search` parameter to filter
    /// by name, email, phone, card code (barcode), or external reference ID. Alternatively, pass
    /// `source` AND `external_id` together to perform an exact lookup by an external reference -
    /// the response will contain at most one customer.
    ///
    /// # Arguments
    ///
    /// * `account_id` - Store (account) ID
    /// * `search` - Search query to filter customers by name, email, phone, card code (barcode), or external reference ID
    /// * `source` - External system slug (e.g. `square`, `shopify`). When combined with `external_id`, performs an exact lookup.
    /// * `external_id` - External system's identifier for the customer. Must be combined with `source`.
    /// * `page` - Page number (defaults to 1)
    /// * `items` - Number of items per page
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
    ///         .customers
    ///         .list(
    ///             1,
    ///             &CustomersListQueryRequest {
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
        request: &CustomersListQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<ListCustomersResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                &format!("api/v1/accounts/{}/customers", account_id),
                None,
                QueryBuilder::new()
                    .string("search", request.search.clone())
                    .string("source", request.source.clone())
                    .string("external_id", request.external_id.clone())
                    .int("page", request.page.clone())
                    .int("items", request.items.clone())
                    .build(),
                options,
            )
            .await
    }

    /// Creates a new customer for the store. Requires `first_name` and at least one of `email` or `phone`.
    /// Optionally enroll the customer in a loyalty card by passing `card_id`, and trigger delivery of
    /// card links (email/SMS) by passing `send_card_links`. When a card with initial stamps is assigned,
    /// those stamps are automatically applied as a welcome bonus.
    ///
    /// Pass `metadata` to attach arbitrary key/value data, and `external_references` to link the
    /// customer to records in other systems (e.g. Square, Shopify). External references are upserted
    /// by `(source, external_id)` so this endpoint is safe to call with the same references twice.
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
    ///         .customers
    ///         .create(
    ///             1,
    ///             &CreateCustomersRequest {
    ///                 customer: CreateCustomersRequestCustomer {
    ///                     first_name: "first_name".to_string(),
    ///                     ..Default::default()
    ///                 },
    ///                 card_id: None,
    ///                 send_card_links: None,
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn create(
        &self,
        account_id: i64,
        request: &CreateCustomersRequest,
        options: Option<RequestOptions>,
    ) -> Result<CreateCustomersResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                &format!("api/v1/accounts/{}/customers", account_id),
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Returns detailed information about a single customer, including all of their
    /// enrolled loyalty cards with stamp progress and wallet pass URLs (`apple_wallet_url`
    /// and `google_wallet_url`) for each card. Also includes `metadata` and
    /// `external_references` so you can sync state with external systems.
    ///
    /// # Arguments
    ///
    /// * `account_id` - Store (account) ID
    /// * `id` - Customer ID
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
    ///     client.customers.get(1, 1, None).await;
    /// }
    /// ```
    pub async fn get(
        &self,
        account_id: i64,
        id: i64,
        options: Option<RequestOptions>,
    ) -> Result<GetCustomersResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                &format!("api/v1/accounts/{}/customers/{}", account_id, id),
                None,
                None,
                options,
            )
            .await
    }

    /// Updates an existing customer's details. To add stamps or redeem rewards, use the
    /// customer cards endpoints instead.
    ///
    /// `metadata` is shallow-merged into the existing metadata. `external_references` are upserted
    /// by `(source, external_id)` - to remove a reference, omit it from subsequent calls and use
    /// a separate `DELETE` workflow (not yet exposed via API; manage in dashboard for now).
    ///
    /// # Arguments
    ///
    /// * `account_id` - Store (account) ID
    /// * `id` - Customer ID
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
    ///         .customers
    ///         .update(
    ///             1,
    ///             1,
    ///             &UpdateCustomersRequest {
    ///                 customer: UpdateCustomersRequestCustomer {
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
        request: &UpdateCustomersRequest,
        options: Option<RequestOptions>,
    ) -> Result<UpdateCustomersResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::PATCH,
                &format!("api/v1/accounts/{}/customers/{}", account_id, id),
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }
}
