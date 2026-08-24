pub use crate::prelude::*;

/// Query parameters for list
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostersListQueryRequest {
    /// Filter posters belonging to a specific card
    #[serde(skip_serializing_if = "Option::is_none")]
    pub card_id: Option<i64>,
    /// When present, return only active posters
    #[serde(skip_serializing_if = "Option::is_none")]
    pub active: Option<String>,
}

impl PostersListQueryRequest {
    pub fn builder() -> PostersListQueryRequestBuilder {
        <PostersListQueryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostersListQueryRequestBuilder {
    card_id: Option<i64>,
    active: Option<String>,
}

impl PostersListQueryRequestBuilder {
    pub fn card_id(mut self, value: i64) -> Self {
        self.card_id = Some(value);
        self
    }

    pub fn active(mut self, value: impl Into<String>) -> Self {
        self.active = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`PostersListQueryRequest`].
    pub fn build(self) -> Result<PostersListQueryRequest, BuildError> {
        Ok(PostersListQueryRequest {
            card_id: self.card_id,
            active: self.active,
        })
    }
}
