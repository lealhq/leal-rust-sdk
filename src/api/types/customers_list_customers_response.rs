pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ListCustomersResponse {
    /// The customers on this page
    #[serde(default)]
    pub customers: Vec<String>,
    #[serde(default)]
    pub pagination: ListCustomersResponsePagination,
}

impl ListCustomersResponse {
    pub fn builder() -> ListCustomersResponseBuilder {
        <ListCustomersResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ListCustomersResponseBuilder {
    customers: Option<Vec<String>>,
    pagination: Option<ListCustomersResponsePagination>,
}

impl ListCustomersResponseBuilder {
    pub fn customers(mut self, value: Vec<String>) -> Self {
        self.customers = Some(value);
        self
    }

    pub fn pagination(mut self, value: ListCustomersResponsePagination) -> Self {
        self.pagination = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ListCustomersResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`customers`](ListCustomersResponseBuilder::customers)
    /// - [`pagination`](ListCustomersResponseBuilder::pagination)
    pub fn build(self) -> Result<ListCustomersResponse, BuildError> {
        Ok(ListCustomersResponse {
            customers: self
                .customers
                .ok_or_else(|| BuildError::missing_field("customers"))?,
            pagination: self
                .pagination
                .ok_or_else(|| BuildError::missing_field("pagination"))?,
        })
    }
}
