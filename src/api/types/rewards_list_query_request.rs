pub use crate::prelude::*;

/// Query parameters for list
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct RewardsListQueryRequest {
    /// Filter rewards belonging to a specific card
    #[serde(skip_serializing_if = "Option::is_none")]
    pub card_id: Option<i64>,
    /// When present, return only active rewards
    #[serde(skip_serializing_if = "Option::is_none")]
    pub active: Option<String>,
}

impl RewardsListQueryRequest {
    pub fn builder() -> RewardsListQueryRequestBuilder {
        <RewardsListQueryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct RewardsListQueryRequestBuilder {
    card_id: Option<i64>,
    active: Option<String>,
}

impl RewardsListQueryRequestBuilder {
    pub fn card_id(mut self, value: i64) -> Self {
        self.card_id = Some(value);
        self
    }

    pub fn active(mut self, value: impl Into<String>) -> Self {
        self.active = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`RewardsListQueryRequest`].
    pub fn build(self) -> Result<RewardsListQueryRequest, BuildError> {
        Ok(RewardsListQueryRequest {
            card_id: self.card_id,
            active: self.active,
        })
    }
}
