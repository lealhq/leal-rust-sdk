pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct RedeemCustomerCardsResponseRedemption {
    /// Redemption ID
    #[serde(default)]
    pub id: i64,
    /// ISO 8601 timestamp of the redemption
    #[serde(default)]
    pub redeemed_at: String,
    /// Reward that was redeemed
    #[serde(default)]
    pub reward_id: i64,
    /// Display name of the reward
    #[serde(default)]
    pub reward_name: String,
    /// Stamps left on the card afterwards
    #[serde(default)]
    pub stamps_remaining: i64,
    /// Stamps deducted from the card
    #[serde(default)]
    pub stamps_spent: i64,
}

impl RedeemCustomerCardsResponseRedemption {
    pub fn builder() -> RedeemCustomerCardsResponseRedemptionBuilder {
        <RedeemCustomerCardsResponseRedemptionBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct RedeemCustomerCardsResponseRedemptionBuilder {
    id: Option<i64>,
    redeemed_at: Option<String>,
    reward_id: Option<i64>,
    reward_name: Option<String>,
    stamps_remaining: Option<i64>,
    stamps_spent: Option<i64>,
}

impl RedeemCustomerCardsResponseRedemptionBuilder {
    pub fn id(mut self, value: i64) -> Self {
        self.id = Some(value);
        self
    }

    pub fn redeemed_at(mut self, value: impl Into<String>) -> Self {
        self.redeemed_at = Some(value.into());
        self
    }

    pub fn reward_id(mut self, value: i64) -> Self {
        self.reward_id = Some(value);
        self
    }

    pub fn reward_name(mut self, value: impl Into<String>) -> Self {
        self.reward_name = Some(value.into());
        self
    }

    pub fn stamps_remaining(mut self, value: i64) -> Self {
        self.stamps_remaining = Some(value);
        self
    }

    pub fn stamps_spent(mut self, value: i64) -> Self {
        self.stamps_spent = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`RedeemCustomerCardsResponseRedemption`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](RedeemCustomerCardsResponseRedemptionBuilder::id)
    /// - [`redeemed_at`](RedeemCustomerCardsResponseRedemptionBuilder::redeemed_at)
    /// - [`reward_id`](RedeemCustomerCardsResponseRedemptionBuilder::reward_id)
    /// - [`reward_name`](RedeemCustomerCardsResponseRedemptionBuilder::reward_name)
    /// - [`stamps_remaining`](RedeemCustomerCardsResponseRedemptionBuilder::stamps_remaining)
    /// - [`stamps_spent`](RedeemCustomerCardsResponseRedemptionBuilder::stamps_spent)
    pub fn build(self) -> Result<RedeemCustomerCardsResponseRedemption, BuildError> {
        Ok(RedeemCustomerCardsResponseRedemption {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            redeemed_at: self
                .redeemed_at
                .ok_or_else(|| BuildError::missing_field("redeemed_at"))?,
            reward_id: self
                .reward_id
                .ok_or_else(|| BuildError::missing_field("reward_id"))?,
            reward_name: self
                .reward_name
                .ok_or_else(|| BuildError::missing_field("reward_name"))?,
            stamps_remaining: self
                .stamps_remaining
                .ok_or_else(|| BuildError::missing_field("stamps_remaining"))?,
            stamps_spent: self
                .stamps_spent
                .ok_or_else(|| BuildError::missing_field("stamps_spent"))?,
        })
    }
}
