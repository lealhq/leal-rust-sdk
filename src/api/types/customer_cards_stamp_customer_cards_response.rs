pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct StampCustomerCardsResponse {
    /// Parent store ID
    #[serde(default)]
    pub account_id: i64,
    /// Link to add or view the pass in Apple Wallet
    #[serde(default)]
    pub apple_wallet_url: String,
    /// Rewards this customer can redeem right now
    #[serde(default)]
    pub available_rewards: Vec<String>,
    /// Loyalty card template ID
    #[serde(default)]
    pub card_id: i64,
    /// Name of the loyalty card
    #[serde(default)]
    pub card_name: String,
    /// ISO 8601 creation timestamp
    #[serde(default)]
    pub created_at: String,
    /// Owning customer ID
    #[serde(default)]
    pub customer_id: i64,
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

impl StampCustomerCardsResponse {
    pub fn builder() -> StampCustomerCardsResponseBuilder {
        <StampCustomerCardsResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct StampCustomerCardsResponseBuilder {
    account_id: Option<i64>,
    apple_wallet_url: Option<String>,
    available_rewards: Option<Vec<String>>,
    card_id: Option<i64>,
    card_name: Option<String>,
    created_at: Option<String>,
    customer_id: Option<i64>,
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

impl StampCustomerCardsResponseBuilder {
    pub fn account_id(mut self, value: i64) -> Self {
        self.account_id = Some(value);
        self
    }

    pub fn apple_wallet_url(mut self, value: impl Into<String>) -> Self {
        self.apple_wallet_url = Some(value.into());
        self
    }

    pub fn available_rewards(mut self, value: Vec<String>) -> Self {
        self.available_rewards = Some(value);
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

    pub fn customer_id(mut self, value: i64) -> Self {
        self.customer_id = Some(value);
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

    /// Consumes the builder and constructs a [`StampCustomerCardsResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`account_id`](StampCustomerCardsResponseBuilder::account_id)
    /// - [`apple_wallet_url`](StampCustomerCardsResponseBuilder::apple_wallet_url)
    /// - [`available_rewards`](StampCustomerCardsResponseBuilder::available_rewards)
    /// - [`card_id`](StampCustomerCardsResponseBuilder::card_id)
    /// - [`card_name`](StampCustomerCardsResponseBuilder::card_name)
    /// - [`created_at`](StampCustomerCardsResponseBuilder::created_at)
    /// - [`customer_id`](StampCustomerCardsResponseBuilder::customer_id)
    /// - [`google_wallet_url`](StampCustomerCardsResponseBuilder::google_wallet_url)
    /// - [`id`](StampCustomerCardsResponseBuilder::id)
    /// - [`issued_at`](StampCustomerCardsResponseBuilder::issued_at)
    /// - [`pass_installed`](StampCustomerCardsResponseBuilder::pass_installed)
    /// - [`progress_percentage`](StampCustomerCardsResponseBuilder::progress_percentage)
    /// - [`stamps_count`](StampCustomerCardsResponseBuilder::stamps_count)
    /// - [`stamps_remaining`](StampCustomerCardsResponseBuilder::stamps_remaining)
    /// - [`status`](StampCustomerCardsResponseBuilder::status)
    /// - [`updated_at`](StampCustomerCardsResponseBuilder::updated_at)
    /// - [`uuid`](StampCustomerCardsResponseBuilder::uuid)
    pub fn build(self) -> Result<StampCustomerCardsResponse, BuildError> {
        Ok(StampCustomerCardsResponse {
            account_id: self
                .account_id
                .ok_or_else(|| BuildError::missing_field("account_id"))?,
            apple_wallet_url: self
                .apple_wallet_url
                .ok_or_else(|| BuildError::missing_field("apple_wallet_url"))?,
            available_rewards: self
                .available_rewards
                .ok_or_else(|| BuildError::missing_field("available_rewards"))?,
            card_id: self
                .card_id
                .ok_or_else(|| BuildError::missing_field("card_id"))?,
            card_name: self
                .card_name
                .ok_or_else(|| BuildError::missing_field("card_name"))?,
            created_at: self
                .created_at
                .ok_or_else(|| BuildError::missing_field("created_at"))?,
            customer_id: self
                .customer_id
                .ok_or_else(|| BuildError::missing_field("customer_id"))?,
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
