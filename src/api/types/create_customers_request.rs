pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct CreateCustomersRequest {
    /// Loyalty card ID to auto-enroll the customer in
    #[serde(skip_serializing_if = "Option::is_none")]
    pub card_id: Option<i64>,
    #[serde(default)]
    pub customer: CreateCustomersRequestCustomer,
    /// When true, sends the card links to the customer via email/SMS after enrollment. Note: even without this flag, the response includes `apple_wallet_url` and `google_wallet_url` in each customer card object so you can deliver them yourself.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub send_card_links: Option<bool>,
}

impl CreateCustomersRequest {
    pub fn builder() -> CreateCustomersRequestBuilder {
        <CreateCustomersRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CreateCustomersRequestBuilder {
    card_id: Option<i64>,
    customer: Option<CreateCustomersRequestCustomer>,
    send_card_links: Option<bool>,
}

impl CreateCustomersRequestBuilder {
    pub fn card_id(mut self, value: i64) -> Self {
        self.card_id = Some(value);
        self
    }

    pub fn customer(mut self, value: CreateCustomersRequestCustomer) -> Self {
        self.customer = Some(value);
        self
    }

    pub fn send_card_links(mut self, value: bool) -> Self {
        self.send_card_links = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`CreateCustomersRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`customer`](CreateCustomersRequestBuilder::customer)
    pub fn build(self) -> Result<CreateCustomersRequest, BuildError> {
        Ok(CreateCustomersRequest {
            card_id: self.card_id,
            customer: self
                .customer
                .ok_or_else(|| BuildError::missing_field("customer"))?,
            send_card_links: self.send_card_links,
        })
    }
}
