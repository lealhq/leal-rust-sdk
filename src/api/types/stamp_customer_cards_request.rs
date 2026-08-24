pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct StampCustomerCardsRequest {
    /// When true, stamp changes bypass notifications
    #[serde(skip_serializing_if = "Option::is_none")]
    pub skip_notifications: Option<bool>,
    /// Number of stamps to add (e.g. 1, 3)
    #[serde(default)]
    pub stamps: i64,
}

impl StampCustomerCardsRequest {
    pub fn builder() -> StampCustomerCardsRequestBuilder {
        <StampCustomerCardsRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct StampCustomerCardsRequestBuilder {
    skip_notifications: Option<bool>,
    stamps: Option<i64>,
}

impl StampCustomerCardsRequestBuilder {
    pub fn skip_notifications(mut self, value: bool) -> Self {
        self.skip_notifications = Some(value);
        self
    }

    pub fn stamps(mut self, value: i64) -> Self {
        self.stamps = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`StampCustomerCardsRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`stamps`](StampCustomerCardsRequestBuilder::stamps)
    pub fn build(self) -> Result<StampCustomerCardsRequest, BuildError> {
        Ok(StampCustomerCardsRequest {
            skip_notifications: self.skip_notifications,
            stamps: self
                .stamps
                .ok_or_else(|| BuildError::missing_field("stamps"))?,
        })
    }
}
