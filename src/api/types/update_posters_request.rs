pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct UpdatePostersRequest {
    #[serde(default)]
    pub poster: UpdatePostersRequestPoster,
}

impl UpdatePostersRequest {
    pub fn builder() -> UpdatePostersRequestBuilder {
        <UpdatePostersRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct UpdatePostersRequestBuilder {
    poster: Option<UpdatePostersRequestPoster>,
}

impl UpdatePostersRequestBuilder {
    pub fn poster(mut self, value: UpdatePostersRequestPoster) -> Self {
        self.poster = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`UpdatePostersRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`poster`](UpdatePostersRequestBuilder::poster)
    pub fn build(self) -> Result<UpdatePostersRequest, BuildError> {
        Ok(UpdatePostersRequest {
            poster: self
                .poster
                .ok_or_else(|| BuildError::missing_field("poster"))?,
        })
    }
}
