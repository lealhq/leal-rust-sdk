pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct UpdateLocationsRequestLocation {
    /// Full street address – automatically geocoded to lat/lng
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address: Option<String>,
    /// Location name
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

impl UpdateLocationsRequestLocation {
    pub fn builder() -> UpdateLocationsRequestLocationBuilder {
        <UpdateLocationsRequestLocationBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct UpdateLocationsRequestLocationBuilder {
    address: Option<String>,
    name: Option<String>,
}

impl UpdateLocationsRequestLocationBuilder {
    pub fn address(mut self, value: impl Into<String>) -> Self {
        self.address = Some(value.into());
        self
    }

    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`UpdateLocationsRequestLocation`].
    pub fn build(self) -> Result<UpdateLocationsRequestLocation, BuildError> {
        Ok(UpdateLocationsRequestLocation {
            address: self.address,
            name: self.name,
        })
    }
}
