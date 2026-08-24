pub use crate::prelude::*;

/// Query parameters for list
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct CustomersListQueryRequest {
    /// Search query to filter customers by name, email, phone, card code (barcode), or external reference ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub search: Option<String>,
    /// External system slug (e.g. `square`, `shopify`). When combined with `external_id`, performs an exact lookup.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source: Option<String>,
    /// External system's identifier for the customer. Must be combined with `source`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub external_id: Option<String>,
    /// Page number (defaults to 1)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page: Option<i64>,
    /// Number of items per page
    #[serde(skip_serializing_if = "Option::is_none")]
    pub items: Option<i64>,
}

impl CustomersListQueryRequest {
    pub fn builder() -> CustomersListQueryRequestBuilder {
        <CustomersListQueryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CustomersListQueryRequestBuilder {
    search: Option<String>,
    source: Option<String>,
    external_id: Option<String>,
    page: Option<i64>,
    items: Option<i64>,
}

impl CustomersListQueryRequestBuilder {
    pub fn search(mut self, value: impl Into<String>) -> Self {
        self.search = Some(value.into());
        self
    }

    pub fn source(mut self, value: impl Into<String>) -> Self {
        self.source = Some(value.into());
        self
    }

    pub fn external_id(mut self, value: impl Into<String>) -> Self {
        self.external_id = Some(value.into());
        self
    }

    pub fn page(mut self, value: i64) -> Self {
        self.page = Some(value);
        self
    }

    pub fn items(mut self, value: i64) -> Self {
        self.items = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`CustomersListQueryRequest`].
    pub fn build(self) -> Result<CustomersListQueryRequest, BuildError> {
        Ok(CustomersListQueryRequest {
            search: self.search,
            source: self.source,
            external_id: self.external_id,
            page: self.page,
            items: self.items,
        })
    }
}
