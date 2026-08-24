pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct RedeemCustomerCardsResponse {
    #[serde(default)]
    pub redemption: RedeemCustomerCardsResponseRedemption,
    /// True when the reward was redeemed
    #[serde(default)]
    pub success: bool,
}

impl RedeemCustomerCardsResponse {
    pub fn builder() -> RedeemCustomerCardsResponseBuilder {
        <RedeemCustomerCardsResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct RedeemCustomerCardsResponseBuilder {
    redemption: Option<RedeemCustomerCardsResponseRedemption>,
    success: Option<bool>,
}

impl RedeemCustomerCardsResponseBuilder {
    pub fn redemption(mut self, value: RedeemCustomerCardsResponseRedemption) -> Self {
        self.redemption = Some(value);
        self
    }

    pub fn success(mut self, value: bool) -> Self {
        self.success = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`RedeemCustomerCardsResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`redemption`](RedeemCustomerCardsResponseBuilder::redemption)
    /// - [`success`](RedeemCustomerCardsResponseBuilder::success)
    pub fn build(self) -> Result<RedeemCustomerCardsResponse, BuildError> {
        Ok(RedeemCustomerCardsResponse {
            redemption: self
                .redemption
                .ok_or_else(|| BuildError::missing_field("redemption"))?,
            success: self
                .success
                .ok_or_else(|| BuildError::missing_field("success"))?,
        })
    }
}
