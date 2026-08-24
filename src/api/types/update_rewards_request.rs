pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct UpdateRewardsRequest {
    #[serde(default)]
    pub reward: UpdateRewardsRequestReward,
}

impl UpdateRewardsRequest {
    pub fn builder() -> UpdateRewardsRequestBuilder {
        <UpdateRewardsRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct UpdateRewardsRequestBuilder {
    reward: Option<UpdateRewardsRequestReward>,
}

impl UpdateRewardsRequestBuilder {
    pub fn reward(mut self, value: UpdateRewardsRequestReward) -> Self {
        self.reward = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`UpdateRewardsRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`reward`](UpdateRewardsRequestBuilder::reward)
    pub fn build(self) -> Result<UpdateRewardsRequest, BuildError> {
        Ok(UpdateRewardsRequest {
            reward: self
                .reward
                .ok_or_else(|| BuildError::missing_field("reward"))?,
        })
    }
}
