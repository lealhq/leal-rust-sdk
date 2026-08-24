pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct CreateRewardsResponse {
    /// Parent store ID
    #[serde(default)]
    pub account_id: i64,
    /// Whether the reward can currently be redeemed
    #[serde(default)]
    pub active: bool,
    /// ID of the loyalty card this reward belongs to
    #[serde(default)]
    pub card_id: i64,
    /// ISO 8601 creation timestamp
    #[serde(default)]
    pub created_at: String,
    /// Longer description of the reward
    #[serde(default)]
    pub description: String,
    /// Unique reward ID
    #[serde(default)]
    pub id: i64,
    /// Display name of the reward
    #[serde(default)]
    pub name: String,
    /// Display order
    #[serde(default)]
    pub position: i64,
    /// Stamps needed before the reward can be redeemed
    #[serde(default)]
    pub stamps_required: i64,
    /// ISO 8601 last-update timestamp
    #[serde(default)]
    pub updated_at: String,
}

impl CreateRewardsResponse {
    pub fn builder() -> CreateRewardsResponseBuilder {
        <CreateRewardsResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CreateRewardsResponseBuilder {
    account_id: Option<i64>,
    active: Option<bool>,
    card_id: Option<i64>,
    created_at: Option<String>,
    description: Option<String>,
    id: Option<i64>,
    name: Option<String>,
    position: Option<i64>,
    stamps_required: Option<i64>,
    updated_at: Option<String>,
}

impl CreateRewardsResponseBuilder {
    pub fn account_id(mut self, value: i64) -> Self {
        self.account_id = Some(value);
        self
    }

    pub fn active(mut self, value: bool) -> Self {
        self.active = Some(value);
        self
    }

    pub fn card_id(mut self, value: i64) -> Self {
        self.card_id = Some(value);
        self
    }

    pub fn created_at(mut self, value: impl Into<String>) -> Self {
        self.created_at = Some(value.into());
        self
    }

    pub fn description(mut self, value: impl Into<String>) -> Self {
        self.description = Some(value.into());
        self
    }

    pub fn id(mut self, value: i64) -> Self {
        self.id = Some(value);
        self
    }

    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    pub fn position(mut self, value: i64) -> Self {
        self.position = Some(value);
        self
    }

    pub fn stamps_required(mut self, value: i64) -> Self {
        self.stamps_required = Some(value);
        self
    }

    pub fn updated_at(mut self, value: impl Into<String>) -> Self {
        self.updated_at = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`CreateRewardsResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`account_id`](CreateRewardsResponseBuilder::account_id)
    /// - [`active`](CreateRewardsResponseBuilder::active)
    /// - [`card_id`](CreateRewardsResponseBuilder::card_id)
    /// - [`created_at`](CreateRewardsResponseBuilder::created_at)
    /// - [`description`](CreateRewardsResponseBuilder::description)
    /// - [`id`](CreateRewardsResponseBuilder::id)
    /// - [`name`](CreateRewardsResponseBuilder::name)
    /// - [`position`](CreateRewardsResponseBuilder::position)
    /// - [`stamps_required`](CreateRewardsResponseBuilder::stamps_required)
    /// - [`updated_at`](CreateRewardsResponseBuilder::updated_at)
    pub fn build(self) -> Result<CreateRewardsResponse, BuildError> {
        Ok(CreateRewardsResponse {
            account_id: self
                .account_id
                .ok_or_else(|| BuildError::missing_field("account_id"))?,
            active: self
                .active
                .ok_or_else(|| BuildError::missing_field("active"))?,
            card_id: self
                .card_id
                .ok_or_else(|| BuildError::missing_field("card_id"))?,
            created_at: self
                .created_at
                .ok_or_else(|| BuildError::missing_field("created_at"))?,
            description: self
                .description
                .ok_or_else(|| BuildError::missing_field("description"))?,
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
            position: self
                .position
                .ok_or_else(|| BuildError::missing_field("position"))?,
            stamps_required: self
                .stamps_required
                .ok_or_else(|| BuildError::missing_field("stamps_required"))?,
            updated_at: self
                .updated_at
                .ok_or_else(|| BuildError::missing_field("updated_at"))?,
        })
    }
}
