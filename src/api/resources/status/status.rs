use crate::api::*;
use crate::{ApiError, ClientConfig, HttpClient, RequestOptions};
use reqwest::Method;

pub struct StatusClient {
    pub http_client: HttpClient,
}

impl StatusClient {
    pub fn new(config: ClientConfig) -> Result<Self, ApiError> {
        Ok(Self {
            http_client: HttpClient::new(config.clone())?,
        })
    }

    /// Returns the status of the API. No authentication required.
    ///
    /// Every response from this API, including this one, carries `RateLimit-Limit`,
    /// `RateLimit-Remaining`, `RateLimit-Reset` and `RateLimit-Policy`. Exceeding
    /// the limit returns 429 with `Retry-After` in seconds.
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
    ///     client.status.check(None).await;
    /// }
    /// ```
    pub async fn check(
        &self,
        options: Option<RequestOptions>,
    ) -> Result<CheckStatusResponse, ApiError> {
        self.http_client
            .execute_request(Method::GET, "api/v1/status", None, None, options)
            .await
    }
}
