pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct CreateLocationsRequestLocation {
    /// Full street address – automatically geocoded to lat/lng
    #[serde(default)]
    pub address: String,
    /// Location name (e.g. 'High Street Branch')
    #[serde(default)]
    pub name: String,
}

impl CreateLocationsRequestLocation {
    pub fn builder() -> CreateLocationsRequestLocationBuilder {
        <CreateLocationsRequestLocationBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CreateLocationsRequestLocationBuilder {
    address: Option<String>,
    name: Option<String>,
}

impl CreateLocationsRequestLocationBuilder {
    pub fn address(mut self, value: impl Into<String>) -> Self {
        self.address = Some(value.into());
        self
    }

    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`CreateLocationsRequestLocation`].
    /// This method will fail if any of the following fields are not set:
    /// - [`address`](CreateLocationsRequestLocationBuilder::address)
    /// - [`name`](CreateLocationsRequestLocationBuilder::name)
    pub fn build(self) -> Result<CreateLocationsRequestLocation, BuildError> {
        Ok(CreateLocationsRequestLocation {
            address: self
                .address
                .ok_or_else(|| BuildError::missing_field("address"))?,
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
        })
    }
}
