pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct ListCustomerCardsResponseItem {
    /// Link to add or view the pass in Apple Wallet
    #[serde(default)]
    pub apple_wallet_url: String,
    /// Loyalty card template ID
    #[serde(default)]
    pub card_id: i64,
    /// Name of the loyalty card
    #[serde(default)]
    pub card_name: String,
    /// ISO 8601 creation timestamp
    #[serde(default)]
    pub created_at: String,
    /// Link to add or view the pass in Google Wallet
    #[serde(default)]
    pub google_wallet_url: String,
    /// Customer card ID
    #[serde(default)]
    pub id: i64,
    /// ISO 8601 timestamp the card was issued
    #[serde(default)]
    pub issued_at: String,
    /// Whether the wallet pass has been installed
    #[serde(default)]
    pub pass_installed: bool,
    /// Completion towards the next reward, 0 to 100
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers")]
    pub progress_percentage: f64,
    /// Stamps collected so far
    #[serde(default)]
    pub stamps_count: i64,
    /// Stamps still needed to complete the card
    #[serde(default)]
    pub stamps_remaining: i64,
    /// Current state of the customer card
    #[serde(default)]
    pub status: String,
    /// ISO 8601 last-update timestamp
    #[serde(default)]
    pub updated_at: String,
    /// Public identifier used in wallet pass URLs
    #[serde(default)]
    pub uuid: String,
}

impl ListCustomerCardsResponseItem {
    pub fn builder() -> ListCustomerCardsResponseItemBuilder {
        <ListCustomerCardsResponseItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ListCustomerCardsResponseItemBuilder {
    apple_wallet_url: Option<String>,
    card_id: Option<i64>,
    card_name: Option<String>,
    created_at: Option<String>,
    google_wallet_url: Option<String>,
    id: Option<i64>,
    issued_at: Option<String>,
    pass_installed: Option<bool>,
    progress_percentage: Option<f64>,
    stamps_count: Option<i64>,
    stamps_remaining: Option<i64>,
    status: Option<String>,
    updated_at: Option<String>,
    uuid: Option<String>,
}

impl ListCustomerCardsResponseItemBuilder {
    pub fn apple_wallet_url(mut self, value: impl Into<String>) -> Self {
        self.apple_wallet_url = Some(value.into());
        self
    }

    pub fn card_id(mut self, value: i64) -> Self {
        self.card_id = Some(value);
        self
    }

    pub fn card_name(mut self, value: impl Into<String>) -> Self {
        self.card_name = Some(value.into());
        self
    }

    pub fn created_at(mut self, value: impl Into<String>) -> Self {
        self.created_at = Some(value.into());
        self
    }

    pub fn google_wallet_url(mut self, value: impl Into<String>) -> Self {
        self.google_wallet_url = Some(value.into());
        self
    }

    pub fn id(mut self, value: i64) -> Self {
        self.id = Some(value);
        self
    }

    pub fn issued_at(mut self, value: impl Into<String>) -> Self {
        self.issued_at = Some(value.into());
        self
    }

    pub fn pass_installed(mut self, value: bool) -> Self {
        self.pass_installed = Some(value);
        self
    }

    pub fn progress_percentage(mut self, value: f64) -> Self {
        self.progress_percentage = Some(value);
        self
    }

    pub fn stamps_count(mut self, value: i64) -> Self {
        self.stamps_count = Some(value);
        self
    }

    pub fn stamps_remaining(mut self, value: i64) -> Self {
        self.stamps_remaining = Some(value);
        self
    }

    pub fn status(mut self, value: impl Into<String>) -> Self {
        self.status = Some(value.into());
        self
    }

    pub fn updated_at(mut self, value: impl Into<String>) -> Self {
        self.updated_at = Some(value.into());
        self
    }

    pub fn uuid(mut self, value: impl Into<String>) -> Self {
        self.uuid = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`ListCustomerCardsResponseItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`apple_wallet_url`](ListCustomerCardsResponseItemBuilder::apple_wallet_url)
    /// - [`card_id`](ListCustomerCardsResponseItemBuilder::card_id)
    /// - [`card_name`](ListCustomerCardsResponseItemBuilder::card_name)
    /// - [`created_at`](ListCustomerCardsResponseItemBuilder::created_at)
    /// - [`google_wallet_url`](ListCustomerCardsResponseItemBuilder::google_wallet_url)
    /// - [`id`](ListCustomerCardsResponseItemBuilder::id)
    /// - [`issued_at`](ListCustomerCardsResponseItemBuilder::issued_at)
    /// - [`pass_installed`](ListCustomerCardsResponseItemBuilder::pass_installed)
    /// - [`progress_percentage`](ListCustomerCardsResponseItemBuilder::progress_percentage)
    /// - [`stamps_count`](ListCustomerCardsResponseItemBuilder::stamps_count)
    /// - [`stamps_remaining`](ListCustomerCardsResponseItemBuilder::stamps_remaining)
    /// - [`status`](ListCustomerCardsResponseItemBuilder::status)
    /// - [`updated_at`](ListCustomerCardsResponseItemBuilder::updated_at)
    /// - [`uuid`](ListCustomerCardsResponseItemBuilder::uuid)
    pub fn build(self) -> Result<ListCustomerCardsResponseItem, BuildError> {
        Ok(ListCustomerCardsResponseItem {
            apple_wallet_url: self
                .apple_wallet_url
                .ok_or_else(|| BuildError::missing_field("apple_wallet_url"))?,
            card_id: self
                .card_id
                .ok_or_else(|| BuildError::missing_field("card_id"))?,
            card_name: self
                .card_name
                .ok_or_else(|| BuildError::missing_field("card_name"))?,
            created_at: self
                .created_at
                .ok_or_else(|| BuildError::missing_field("created_at"))?,
            google_wallet_url: self
                .google_wallet_url
                .ok_or_else(|| BuildError::missing_field("google_wallet_url"))?,
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            issued_at: self
                .issued_at
                .ok_or_else(|| BuildError::missing_field("issued_at"))?,
            pass_installed: self
                .pass_installed
                .ok_or_else(|| BuildError::missing_field("pass_installed"))?,
            progress_percentage: self
                .progress_percentage
                .ok_or_else(|| BuildError::missing_field("progress_percentage"))?,
            stamps_count: self
                .stamps_count
                .ok_or_else(|| BuildError::missing_field("stamps_count"))?,
            stamps_remaining: self
                .stamps_remaining
                .ok_or_else(|| BuildError::missing_field("stamps_remaining"))?,
            status: self
                .status
                .ok_or_else(|| BuildError::missing_field("status"))?,
            updated_at: self
                .updated_at
                .ok_or_else(|| BuildError::missing_field("updated_at"))?,
            uuid: self.uuid.ok_or_else(|| BuildError::missing_field("uuid"))?,
        })
    }
}
