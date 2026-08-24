pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ListCustomersResponsePagination {
    /// Total customers matching the query
    #[serde(default)]
    pub count: i64,
    /// Customers per page
    #[serde(default)]
    pub items: i64,
    /// Current page number
    #[serde(default)]
    pub page: i64,
    /// Total number of pages
    #[serde(default)]
    pub pages: i64,
}

impl ListCustomersResponsePagination {
    pub fn builder() -> ListCustomersResponsePaginationBuilder {
        <ListCustomersResponsePaginationBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ListCustomersResponsePaginationBuilder {
    count: Option<i64>,
    items: Option<i64>,
    page: Option<i64>,
    pages: Option<i64>,
}

impl ListCustomersResponsePaginationBuilder {
    pub fn count(mut self, value: i64) -> Self {
        self.count = Some(value);
        self
    }

    pub fn items(mut self, value: i64) -> Self {
        self.items = Some(value);
        self
    }

    pub fn page(mut self, value: i64) -> Self {
        self.page = Some(value);
        self
    }

    pub fn pages(mut self, value: i64) -> Self {
        self.pages = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ListCustomersResponsePagination`].
    /// This method will fail if any of the following fields are not set:
    /// - [`count`](ListCustomersResponsePaginationBuilder::count)
    /// - [`items`](ListCustomersResponsePaginationBuilder::items)
    /// - [`page`](ListCustomersResponsePaginationBuilder::page)
    /// - [`pages`](ListCustomersResponsePaginationBuilder::pages)
    pub fn build(self) -> Result<ListCustomersResponsePagination, BuildError> {
        Ok(ListCustomersResponsePagination {
            count: self
                .count
                .ok_or_else(|| BuildError::missing_field("count"))?,
            items: self
                .items
                .ok_or_else(|| BuildError::missing_field("items"))?,
            page: self.page.ok_or_else(|| BuildError::missing_field("page"))?,
            pages: self
                .pages
                .ok_or_else(|| BuildError::missing_field("pages"))?,
        })
    }
}
