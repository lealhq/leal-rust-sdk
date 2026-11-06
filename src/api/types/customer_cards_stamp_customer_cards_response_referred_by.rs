pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct StampCustomerCardsResponseReferredBy {
    /// The inviting customer's card
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers")]
    pub customer_card_id: f64,
    /// The customer who invited them
    #[serde(default)]
    pub customer_id: i64,
    /// Referral ID
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers")]
    pub referral_id: f64,
    /// ISO 8601 timestamp the referral paid out
    #[serde(default)]
    pub rewarded_at: String,
    /// 'pending' until the store first stamps this card for a visit, then 'rewarded'
    #[serde(default)]
    pub status: String,
}

impl StampCustomerCardsResponseReferredBy {
    pub fn builder() -> StampCustomerCardsResponseReferredByBuilder {
        <StampCustomerCardsResponseReferredByBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct StampCustomerCardsResponseReferredByBuilder {
    customer_card_id: Option<f64>,
    customer_id: Option<i64>,
    referral_id: Option<f64>,
    rewarded_at: Option<String>,
    status: Option<String>,
}

impl StampCustomerCardsResponseReferredByBuilder {
    pub fn customer_card_id(mut self, value: f64) -> Self {
        self.customer_card_id = Some(value);
        self
    }

    pub fn customer_id(mut self, value: i64) -> Self {
        self.customer_id = Some(value);
        self
    }

    pub fn referral_id(mut self, value: f64) -> Self {
        self.referral_id = Some(value);
        self
    }

    pub fn rewarded_at(mut self, value: impl Into<String>) -> Self {
        self.rewarded_at = Some(value.into());
        self
    }

    pub fn status(mut self, value: impl Into<String>) -> Self {
        self.status = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`StampCustomerCardsResponseReferredBy`].
    /// This method will fail if any of the following fields are not set:
    /// - [`customer_card_id`](StampCustomerCardsResponseReferredByBuilder::customer_card_id)
    /// - [`customer_id`](StampCustomerCardsResponseReferredByBuilder::customer_id)
    /// - [`referral_id`](StampCustomerCardsResponseReferredByBuilder::referral_id)
    /// - [`rewarded_at`](StampCustomerCardsResponseReferredByBuilder::rewarded_at)
    /// - [`status`](StampCustomerCardsResponseReferredByBuilder::status)
    pub fn build(self) -> Result<StampCustomerCardsResponseReferredBy, BuildError> {
        Ok(StampCustomerCardsResponseReferredBy {
            customer_card_id: self
                .customer_card_id
                .ok_or_else(|| BuildError::missing_field("customer_card_id"))?,
            customer_id: self
                .customer_id
                .ok_or_else(|| BuildError::missing_field("customer_id"))?,
            referral_id: self
                .referral_id
                .ok_or_else(|| BuildError::missing_field("referral_id"))?,
            rewarded_at: self
                .rewarded_at
                .ok_or_else(|| BuildError::missing_field("rewarded_at"))?,
            status: self
                .status
                .ok_or_else(|| BuildError::missing_field("status"))?,
        })
    }
}
