pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct RedeemCustomerCardsRequest {
    /// Reward ID to redeem
    #[serde(default)]
    pub reward_id: i64,
}

impl RedeemCustomerCardsRequest {
    pub fn builder() -> RedeemCustomerCardsRequestBuilder {
        <RedeemCustomerCardsRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct RedeemCustomerCardsRequestBuilder {
    reward_id: Option<i64>,
}

impl RedeemCustomerCardsRequestBuilder {
    pub fn reward_id(mut self, value: i64) -> Self {
        self.reward_id = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`RedeemCustomerCardsRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`reward_id`](RedeemCustomerCardsRequestBuilder::reward_id)
    pub fn build(self) -> Result<RedeemCustomerCardsRequest, BuildError> {
        Ok(RedeemCustomerCardsRequest {
            reward_id: self
                .reward_id
                .ok_or_else(|| BuildError::missing_field("reward_id"))?,
        })
    }
}
