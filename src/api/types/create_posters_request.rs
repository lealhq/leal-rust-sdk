pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct CreatePostersRequest {
    #[serde(default)]
    pub poster: CreatePostersRequestPoster,
}

impl CreatePostersRequest {
    pub fn builder() -> CreatePostersRequestBuilder {
        <CreatePostersRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CreatePostersRequestBuilder {
    poster: Option<CreatePostersRequestPoster>,
}

impl CreatePostersRequestBuilder {
    pub fn poster(mut self, value: CreatePostersRequestPoster) -> Self {
        self.poster = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`CreatePostersRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`poster`](CreatePostersRequestBuilder::poster)
    pub fn build(self) -> Result<CreatePostersRequest, BuildError> {
        Ok(CreatePostersRequest {
            poster: self
                .poster
                .ok_or_else(|| BuildError::missing_field("poster"))?,
        })
    }
}
