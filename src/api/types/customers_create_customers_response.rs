pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct CreateCustomersResponse {
    /// Parent store ID
    #[serde(default)]
    pub account_id: i64,
    /// Birthday as YYYY-MM-DD
    #[serde(default)]
    pub birthday: String,
    /// ISO 8601 creation timestamp
    #[serde(default)]
    pub created_at: String,
    /// Cards this customer is enrolled on
    #[serde(default)]
    pub customer_cards: Vec<String>,
    /// Email address, unique per store
    #[serde(default)]
    pub email: String,
    /// Links to records in other systems
    #[serde(default)]
    pub external_references: Vec<String>,
    /// First name
    #[serde(default)]
    pub first_name: String,
    /// Unique customer ID
    #[serde(default)]
    pub id: i64,
    /// Last name
    #[serde(default)]
    pub last_name: String,
    /// Free form per customer data
    #[serde(default)]
    pub metadata: HashMap<String, serde_json::Value>,
    /// Phone number, unique per store
    #[serde(default)]
    pub phone: String,
    /// Total stamps across every card
    #[serde(default)]
    pub stamp_count: i64,
    /// ISO 8601 last-update timestamp
    #[serde(default)]
    pub updated_at: String,
}

impl CreateCustomersResponse {
    pub fn builder() -> CreateCustomersResponseBuilder {
        <CreateCustomersResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CreateCustomersResponseBuilder {
    account_id: Option<i64>,
    birthday: Option<String>,
    created_at: Option<String>,
    customer_cards: Option<Vec<String>>,
    email: Option<String>,
    external_references: Option<Vec<String>>,
    first_name: Option<String>,
    id: Option<i64>,
    last_name: Option<String>,
    metadata: Option<HashMap<String, serde_json::Value>>,
    phone: Option<String>,
    stamp_count: Option<i64>,
    updated_at: Option<String>,
}

impl CreateCustomersResponseBuilder {
    pub fn account_id(mut self, value: i64) -> Self {
        self.account_id = Some(value);
        self
    }

    pub fn birthday(mut self, value: impl Into<String>) -> Self {
        self.birthday = Some(value.into());
        self
    }

    pub fn created_at(mut self, value: impl Into<String>) -> Self {
        self.created_at = Some(value.into());
        self
    }

    pub fn customer_cards(mut self, value: Vec<String>) -> Self {
        self.customer_cards = Some(value);
        self
    }

    pub fn email(mut self, value: impl Into<String>) -> Self {
        self.email = Some(value.into());
        self
    }

    pub fn external_references(mut self, value: Vec<String>) -> Self {
        self.external_references = Some(value);
        self
    }

    pub fn first_name(mut self, value: impl Into<String>) -> Self {
        self.first_name = Some(value.into());
        self
    }

    pub fn id(mut self, value: i64) -> Self {
        self.id = Some(value);
        self
    }

    pub fn last_name(mut self, value: impl Into<String>) -> Self {
        self.last_name = Some(value.into());
        self
    }

    pub fn metadata(mut self, value: HashMap<String, serde_json::Value>) -> Self {
        self.metadata = Some(value);
        self
    }

    pub fn phone(mut self, value: impl Into<String>) -> Self {
        self.phone = Some(value.into());
        self
    }

    pub fn stamp_count(mut self, value: i64) -> Self {
        self.stamp_count = Some(value);
        self
    }

    pub fn updated_at(mut self, value: impl Into<String>) -> Self {
        self.updated_at = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`CreateCustomersResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`account_id`](CreateCustomersResponseBuilder::account_id)
    /// - [`birthday`](CreateCustomersResponseBuilder::birthday)
    /// - [`created_at`](CreateCustomersResponseBuilder::created_at)
    /// - [`customer_cards`](CreateCustomersResponseBuilder::customer_cards)
    /// - [`email`](CreateCustomersResponseBuilder::email)
    /// - [`external_references`](CreateCustomersResponseBuilder::external_references)
    /// - [`first_name`](CreateCustomersResponseBuilder::first_name)
    /// - [`id`](CreateCustomersResponseBuilder::id)
    /// - [`last_name`](CreateCustomersResponseBuilder::last_name)
    /// - [`metadata`](CreateCustomersResponseBuilder::metadata)
    /// - [`phone`](CreateCustomersResponseBuilder::phone)
    /// - [`stamp_count`](CreateCustomersResponseBuilder::stamp_count)
    /// - [`updated_at`](CreateCustomersResponseBuilder::updated_at)
    pub fn build(self) -> Result<CreateCustomersResponse, BuildError> {
        Ok(CreateCustomersResponse {
            account_id: self
                .account_id
                .ok_or_else(|| BuildError::missing_field("account_id"))?,
            birthday: self
                .birthday
                .ok_or_else(|| BuildError::missing_field("birthday"))?,
            created_at: self
                .created_at
                .ok_or_else(|| BuildError::missing_field("created_at"))?,
            customer_cards: self
                .customer_cards
                .ok_or_else(|| BuildError::missing_field("customer_cards"))?,
            email: self
                .email
                .ok_or_else(|| BuildError::missing_field("email"))?,
            external_references: self
                .external_references
                .ok_or_else(|| BuildError::missing_field("external_references"))?,
            first_name: self
                .first_name
                .ok_or_else(|| BuildError::missing_field("first_name"))?,
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            last_name: self
                .last_name
                .ok_or_else(|| BuildError::missing_field("last_name"))?,
            metadata: self
                .metadata
                .ok_or_else(|| BuildError::missing_field("metadata"))?,
            phone: self
                .phone
                .ok_or_else(|| BuildError::missing_field("phone"))?,
            stamp_count: self
                .stamp_count
                .ok_or_else(|| BuildError::missing_field("stamp_count"))?,
            updated_at: self
                .updated_at
                .ok_or_else(|| BuildError::missing_field("updated_at"))?,
        })
    }
}
