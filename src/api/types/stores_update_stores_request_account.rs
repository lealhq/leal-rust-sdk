pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct UpdateStoresRequestAccount {
    /// Internal account name
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Public-facing store name shown to customers
    #[serde(skip_serializing_if = "Option::is_none")]
    pub store_name: Option<String>,
}

impl UpdateStoresRequestAccount {
    pub fn builder() -> UpdateStoresRequestAccountBuilder {
        <UpdateStoresRequestAccountBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct UpdateStoresRequestAccountBuilder {
    name: Option<String>,
    store_name: Option<String>,
}

impl UpdateStoresRequestAccountBuilder {
    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    pub fn store_name(mut self, value: impl Into<String>) -> Self {
        self.store_name = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`UpdateStoresRequestAccount`].
    pub fn build(self) -> Result<UpdateStoresRequestAccount, BuildError> {
        Ok(UpdateStoresRequestAccount {
            name: self.name,
            store_name: self.store_name,
        })
    }
}
