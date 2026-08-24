pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct UpdateLocationsResponse {
    /// Parent store ID
    #[serde(default)]
    pub account_id: i64,
    /// Full street address
    #[serde(default)]
    pub address: String,
    /// ISO 8601 creation timestamp
    #[serde(default)]
    pub created_at: String,
    /// Unique location ID
    #[serde(default)]
    pub id: i64,
    /// Geocoded latitude (auto-derived from address)
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers")]
    pub latitude: f64,
    /// Geocoded longitude (auto-derived from address)
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers")]
    pub longitude: f64,
    /// Location name (e.g. 'Downtown Branch')
    #[serde(default)]
    pub name: String,
    /// ISO 8601 last-update timestamp
    #[serde(default)]
    pub updated_at: String,
}

impl UpdateLocationsResponse {
    pub fn builder() -> UpdateLocationsResponseBuilder {
        <UpdateLocationsResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct UpdateLocationsResponseBuilder {
    account_id: Option<i64>,
    address: Option<String>,
    created_at: Option<String>,
    id: Option<i64>,
    latitude: Option<f64>,
    longitude: Option<f64>,
    name: Option<String>,
    updated_at: Option<String>,
}

impl UpdateLocationsResponseBuilder {
    pub fn account_id(mut self, value: i64) -> Self {
        self.account_id = Some(value);
        self
    }

    pub fn address(mut self, value: impl Into<String>) -> Self {
        self.address = Some(value.into());
        self
    }

    pub fn created_at(mut self, value: impl Into<String>) -> Self {
        self.created_at = Some(value.into());
        self
    }

    pub fn id(mut self, value: i64) -> Self {
        self.id = Some(value);
        self
    }

    pub fn latitude(mut self, value: f64) -> Self {
        self.latitude = Some(value);
        self
    }

    pub fn longitude(mut self, value: f64) -> Self {
        self.longitude = Some(value);
        self
    }

    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    pub fn updated_at(mut self, value: impl Into<String>) -> Self {
        self.updated_at = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`UpdateLocationsResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`account_id`](UpdateLocationsResponseBuilder::account_id)
    /// - [`address`](UpdateLocationsResponseBuilder::address)
    /// - [`created_at`](UpdateLocationsResponseBuilder::created_at)
    /// - [`id`](UpdateLocationsResponseBuilder::id)
    /// - [`latitude`](UpdateLocationsResponseBuilder::latitude)
    /// - [`longitude`](UpdateLocationsResponseBuilder::longitude)
    /// - [`name`](UpdateLocationsResponseBuilder::name)
    /// - [`updated_at`](UpdateLocationsResponseBuilder::updated_at)
    pub fn build(self) -> Result<UpdateLocationsResponse, BuildError> {
        Ok(UpdateLocationsResponse {
            account_id: self
                .account_id
                .ok_or_else(|| BuildError::missing_field("account_id"))?,
            address: self
                .address
                .ok_or_else(|| BuildError::missing_field("address"))?,
            created_at: self
                .created_at
                .ok_or_else(|| BuildError::missing_field("created_at"))?,
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            latitude: self
                .latitude
                .ok_or_else(|| BuildError::missing_field("latitude"))?,
            longitude: self
                .longitude
                .ok_or_else(|| BuildError::missing_field("longitude"))?,
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
            updated_at: self
                .updated_at
                .ok_or_else(|| BuildError::missing_field("updated_at"))?,
        })
    }
}
