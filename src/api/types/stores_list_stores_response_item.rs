pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ListStoresResponseItem {
    /// Number of loyalty card templates
    #[serde(default)]
    pub cards_count: i64,
    /// ISO 8601 creation timestamp
    #[serde(default)]
    pub created_at: String,
    /// Number of enrolled customers
    #[serde(default)]
    pub customers_count: i64,
    /// Resolved display name (store_name if present, otherwise name)
    #[serde(default)]
    pub display_store_name: String,
    /// Unique store ID
    #[serde(default)]
    pub id: i64,
    /// Number of physical locations
    #[serde(default)]
    pub locations_count: i64,
    /// Internal account name
    #[serde(default)]
    pub name: String,
    /// Whether this is the user's personal account
    #[serde(default)]
    pub personal: bool,
    /// Number of QR signup posters
    #[serde(default)]
    pub posters_count: i64,
    /// Public-facing store name
    #[serde(default)]
    pub store_name: String,
    /// ISO 8601 last-update timestamp
    #[serde(default)]
    pub updated_at: String,
}

impl ListStoresResponseItem {
    pub fn builder() -> ListStoresResponseItemBuilder {
        <ListStoresResponseItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ListStoresResponseItemBuilder {
    cards_count: Option<i64>,
    created_at: Option<String>,
    customers_count: Option<i64>,
    display_store_name: Option<String>,
    id: Option<i64>,
    locations_count: Option<i64>,
    name: Option<String>,
    personal: Option<bool>,
    posters_count: Option<i64>,
    store_name: Option<String>,
    updated_at: Option<String>,
}

impl ListStoresResponseItemBuilder {
    pub fn cards_count(mut self, value: i64) -> Self {
        self.cards_count = Some(value);
        self
    }

    pub fn created_at(mut self, value: impl Into<String>) -> Self {
        self.created_at = Some(value.into());
        self
    }

    pub fn customers_count(mut self, value: i64) -> Self {
        self.customers_count = Some(value);
        self
    }

    pub fn display_store_name(mut self, value: impl Into<String>) -> Self {
        self.display_store_name = Some(value.into());
        self
    }

    pub fn id(mut self, value: i64) -> Self {
        self.id = Some(value);
        self
    }

    pub fn locations_count(mut self, value: i64) -> Self {
        self.locations_count = Some(value);
        self
    }

    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    pub fn personal(mut self, value: bool) -> Self {
        self.personal = Some(value);
        self
    }

    pub fn posters_count(mut self, value: i64) -> Self {
        self.posters_count = Some(value);
        self
    }

    pub fn store_name(mut self, value: impl Into<String>) -> Self {
        self.store_name = Some(value.into());
        self
    }

    pub fn updated_at(mut self, value: impl Into<String>) -> Self {
        self.updated_at = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`ListStoresResponseItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`cards_count`](ListStoresResponseItemBuilder::cards_count)
    /// - [`created_at`](ListStoresResponseItemBuilder::created_at)
    /// - [`customers_count`](ListStoresResponseItemBuilder::customers_count)
    /// - [`display_store_name`](ListStoresResponseItemBuilder::display_store_name)
    /// - [`id`](ListStoresResponseItemBuilder::id)
    /// - [`locations_count`](ListStoresResponseItemBuilder::locations_count)
    /// - [`name`](ListStoresResponseItemBuilder::name)
    /// - [`personal`](ListStoresResponseItemBuilder::personal)
    /// - [`posters_count`](ListStoresResponseItemBuilder::posters_count)
    /// - [`store_name`](ListStoresResponseItemBuilder::store_name)
    /// - [`updated_at`](ListStoresResponseItemBuilder::updated_at)
    pub fn build(self) -> Result<ListStoresResponseItem, BuildError> {
        Ok(ListStoresResponseItem {
            cards_count: self
                .cards_count
                .ok_or_else(|| BuildError::missing_field("cards_count"))?,
            created_at: self
                .created_at
                .ok_or_else(|| BuildError::missing_field("created_at"))?,
            customers_count: self
                .customers_count
                .ok_or_else(|| BuildError::missing_field("customers_count"))?,
            display_store_name: self
                .display_store_name
                .ok_or_else(|| BuildError::missing_field("display_store_name"))?,
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            locations_count: self
                .locations_count
                .ok_or_else(|| BuildError::missing_field("locations_count"))?,
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
            personal: self
                .personal
                .ok_or_else(|| BuildError::missing_field("personal"))?,
            posters_count: self
                .posters_count
                .ok_or_else(|| BuildError::missing_field("posters_count"))?,
            store_name: self
                .store_name
                .ok_or_else(|| BuildError::missing_field("store_name"))?,
            updated_at: self
                .updated_at
                .ok_or_else(|| BuildError::missing_field("updated_at"))?,
        })
    }
}
