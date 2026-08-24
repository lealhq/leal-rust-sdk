pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct CreateRewardsRequest {
    #[serde(default)]
    pub reward: CreateRewardsRequestReward,
}

impl CreateRewardsRequest {
    pub fn builder() -> CreateRewardsRequestBuilder {
        <CreateRewardsRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CreateRewardsRequestBuilder {
    reward: Option<CreateRewardsRequestReward>,
}

impl CreateRewardsRequestBuilder {
    pub fn reward(mut self, value: CreateRewardsRequestReward) -> Self {
        self.reward = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`CreateRewardsRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`reward`](CreateRewardsRequestBuilder::reward)
    pub fn build(self) -> Result<CreateRewardsRequest, BuildError> {
        Ok(CreateRewardsRequest {
            reward: self
                .reward
                .ok_or_else(|| BuildError::missing_field("reward"))?,
        })
    }
}
