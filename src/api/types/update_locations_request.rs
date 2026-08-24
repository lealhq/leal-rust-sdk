pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct UpdateLocationsRequest {
    #[serde(default)]
    pub location: UpdateLocationsRequestLocation,
}

impl UpdateLocationsRequest {
    pub fn builder() -> UpdateLocationsRequestBuilder {
        <UpdateLocationsRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct UpdateLocationsRequestBuilder {
    location: Option<UpdateLocationsRequestLocation>,
}

impl UpdateLocationsRequestBuilder {
    pub fn location(mut self, value: UpdateLocationsRequestLocation) -> Self {
        self.location = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`UpdateLocationsRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`location`](UpdateLocationsRequestBuilder::location)
    pub fn build(self) -> Result<UpdateLocationsRequest, BuildError> {
        Ok(UpdateLocationsRequest {
            location: self
                .location
                .ok_or_else(|| BuildError::missing_field("location"))?,
        })
    }
}
