pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct CreateLocationsRequest {
    #[serde(default)]
    pub location: CreateLocationsRequestLocation,
}

impl CreateLocationsRequest {
    pub fn builder() -> CreateLocationsRequestBuilder {
        <CreateLocationsRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CreateLocationsRequestBuilder {
    location: Option<CreateLocationsRequestLocation>,
}

impl CreateLocationsRequestBuilder {
    pub fn location(mut self, value: CreateLocationsRequestLocation) -> Self {
        self.location = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`CreateLocationsRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`location`](CreateLocationsRequestBuilder::location)
    pub fn build(self) -> Result<CreateLocationsRequest, BuildError> {
        Ok(CreateLocationsRequest {
            location: self
                .location
                .ok_or_else(|| BuildError::missing_field("location"))?,
        })
    }
}
