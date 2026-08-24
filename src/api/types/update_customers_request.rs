pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct UpdateCustomersRequest {
    #[serde(default)]
    pub customer: UpdateCustomersRequestCustomer,
}

impl UpdateCustomersRequest {
    pub fn builder() -> UpdateCustomersRequestBuilder {
        <UpdateCustomersRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct UpdateCustomersRequestBuilder {
    customer: Option<UpdateCustomersRequestCustomer>,
}

impl UpdateCustomersRequestBuilder {
    pub fn customer(mut self, value: UpdateCustomersRequestCustomer) -> Self {
        self.customer = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`UpdateCustomersRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`customer`](UpdateCustomersRequestBuilder::customer)
    pub fn build(self) -> Result<UpdateCustomersRequest, BuildError> {
        Ok(UpdateCustomersRequest {
            customer: self
                .customer
                .ok_or_else(|| BuildError::missing_field("customer"))?,
        })
    }
}
