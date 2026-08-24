pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct UpdateStoresRequest {
    #[serde(default)]
    pub account: UpdateStoresRequestAccount,
}

impl UpdateStoresRequest {
    pub fn builder() -> UpdateStoresRequestBuilder {
        <UpdateStoresRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct UpdateStoresRequestBuilder {
    account: Option<UpdateStoresRequestAccount>,
}

impl UpdateStoresRequestBuilder {
    pub fn account(mut self, value: UpdateStoresRequestAccount) -> Self {
        self.account = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`UpdateStoresRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`account`](UpdateStoresRequestBuilder::account)
    pub fn build(self) -> Result<UpdateStoresRequest, BuildError> {
        Ok(UpdateStoresRequest {
            account: self
                .account
                .ok_or_else(|| BuildError::missing_field("account"))?,
        })
    }
}
