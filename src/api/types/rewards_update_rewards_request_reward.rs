pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct UpdateRewardsRequestReward {
    /// Whether the reward is active and redeemable
    #[serde(skip_serializing_if = "Option::is_none")]
    pub active: Option<bool>,
    /// Detailed description of the reward
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// Display name of the reward
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Display order position
    #[serde(skip_serializing_if = "Option::is_none")]
    pub position: Option<i64>,
    /// Number of stamps needed to unlock this reward (must be > 0)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stamps_required: Option<i64>,
}

impl UpdateRewardsRequestReward {
    pub fn builder() -> UpdateRewardsRequestRewardBuilder {
        <UpdateRewardsRequestRewardBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct UpdateRewardsRequestRewardBuilder {
    active: Option<bool>,
    description: Option<String>,
    name: Option<String>,
    position: Option<i64>,
    stamps_required: Option<i64>,
}

impl UpdateRewardsRequestRewardBuilder {
    pub fn active(mut self, value: bool) -> Self {
        self.active = Some(value);
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

    /// Consumes the builder and constructs a [`UpdateRewardsRequestReward`].
    pub fn build(self) -> Result<UpdateRewardsRequestReward, BuildError> {
        Ok(UpdateRewardsRequestReward {
            active: self.active,
            description: self.description,
            name: self.name,
            position: self.position,
            stamps_required: self.stamps_required,
        })
    }
}
