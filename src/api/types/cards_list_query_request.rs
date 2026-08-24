pub use crate::prelude::*;

/// Query parameters for list
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct CardsListQueryRequest {
    /// Filter cards by archive status. Default: active only.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scope: Option<String>,
}

impl CardsListQueryRequest {
    pub fn builder() -> CardsListQueryRequestBuilder {
        <CardsListQueryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CardsListQueryRequestBuilder {
    scope: Option<String>,
}

impl CardsListQueryRequestBuilder {
    pub fn scope(mut self, value: impl Into<String>) -> Self {
        self.scope = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`CardsListQueryRequest`].
    pub fn build(self) -> Result<CardsListQueryRequest, BuildError> {
        Ok(CardsListQueryRequest { scope: self.scope })
    }
}
