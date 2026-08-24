pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct CreateRewardsRequestReward {
    /// Whether the reward is active and redeemable (defaults to true)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub active: Option<bool>,
    /// ID of the loyalty card this reward belongs to
    #[serde(default)]
    pub card_id: i64,
    /// Detailed description of the reward
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// Display name of the reward (e.g. 'Free Coffee')
    #[serde(default)]
    pub name: String,
    /// Display order position (lower numbers appear first)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub position: Option<i64>,
    /// Number of stamps needed to unlock this reward (must be > 0)
    #[serde(default)]
    pub stamps_required: i64,
}

impl CreateRewardsRequestReward {
    pub fn builder() -> CreateRewardsRequestRewardBuilder {
        <CreateRewardsRequestRewardBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CreateRewardsRequestRewardBuilder {
    active: Option<bool>,
    card_id: Option<i64>,
    description: Option<String>,
    name: Option<String>,
    position: Option<i64>,
    stamps_required: Option<i64>,
}

impl CreateRewardsRequestRewardBuilder {
    pub fn active(mut self, value: bool) -> Self {
        self.active = Some(value);
        self
    }

    pub fn card_id(mut self, value: i64) -> Self {
        self.card_id = Some(value);
        self
    }

    pub fn description(mut self, value: impl Into<String>) -> Self {
        self.description = Some(value.into());
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

    /// Consumes the builder and constructs a [`CreateRewardsRequestReward`].
    /// This method will fail if any of the following fields are not set:
    /// - [`card_id`](CreateRewardsRequestRewardBuilder::card_id)
    /// - [`name`](CreateRewardsRequestRewardBuilder::name)
    /// - [`stamps_required`](CreateRewardsRequestRewardBuilder::stamps_required)
    pub fn build(self) -> Result<CreateRewardsRequestReward, BuildError> {
        Ok(CreateRewardsRequestReward {
            active: self.active,
            card_id: self
                .card_id
                .ok_or_else(|| BuildError::missing_field("card_id"))?,
            description: self.description,
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
            position: self.position,
            stamps_required: self
                .stamps_required
                .ok_or_else(|| BuildError::missing_field("stamps_required"))?,
        })
    }
}
