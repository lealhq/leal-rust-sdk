pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct CreateCustomersRequestCustomer {
    /// Customer's birthday (YYYY-MM-DD)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub birthday: Option<String>,
    /// Customer's email address (unique per store; required if phone is blank)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    /// Array of `{source, external_id, metadata}` objects linking this customer to records in external systems
    #[serde(skip_serializing_if = "Option::is_none")]
    pub external_references: Option<Vec<String>>,
    /// Customer's first name
    #[serde(default)]
    pub first_name: String,
    /// Customer's last name
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_name: Option<String>,
    /// Free-form JSON object of additional per-customer attributes
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<HashMap<String, serde_json::Value>>,
    /// Customer's phone number (unique per store; required if email is blank)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phone: Option<String>,
}

impl CreateCustomersRequestCustomer {
    pub fn builder() -> CreateCustomersRequestCustomerBuilder {
        <CreateCustomersRequestCustomerBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CreateCustomersRequestCustomerBuilder {
    birthday: Option<String>,
    email: Option<String>,
    external_references: Option<Vec<String>>,
    first_name: Option<String>,
    last_name: Option<String>,
    metadata: Option<HashMap<String, serde_json::Value>>,
    phone: Option<String>,
}

impl CreateCustomersRequestCustomerBuilder {
    pub fn birthday(mut self, value: impl Into<String>) -> Self {
        self.birthday = Some(value.into());
        self
    }

    pub fn email(mut self, value: impl Into<String>) -> Self {
        self.email = Some(value.into());
        self
    }

    pub fn external_references(mut self, value: Vec<String>) -> Self {
        self.external_references = Some(value);
        self
    }

    pub fn first_name(mut self, value: impl Into<String>) -> Self {
        self.first_name = Some(value.into());
        self
    }

    pub fn last_name(mut self, value: impl Into<String>) -> Self {
        self.last_name = Some(value.into());
        self
    }

    pub fn metadata(mut self, value: HashMap<String, serde_json::Value>) -> Self {
        self.metadata = Some(value);
        self
    }

    pub fn phone(mut self, value: impl Into<String>) -> Self {
        self.phone = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`CreateCustomersRequestCustomer`].
    /// This method will fail if any of the following fields are not set:
    /// - [`first_name`](CreateCustomersRequestCustomerBuilder::first_name)
    pub fn build(self) -> Result<CreateCustomersRequestCustomer, BuildError> {
        Ok(CreateCustomersRequestCustomer {
            birthday: self.birthday,
            email: self.email,
            external_references: self.external_references,
            first_name: self
                .first_name
                .ok_or_else(|| BuildError::missing_field("first_name"))?,
            last_name: self.last_name,
            metadata: self.metadata,
            phone: self.phone,
        })
    }
}
