pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct CreateCardsRequest {
    #[serde(default)]
    pub card: CreateCardsRequestCard,
}

impl CreateCardsRequest {
    pub fn builder() -> CreateCardsRequestBuilder {
        <CreateCardsRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CreateCardsRequestBuilder {
    card: Option<CreateCardsRequestCard>,
}

impl CreateCardsRequestBuilder {
    pub fn card(mut self, value: CreateCardsRequestCard) -> Self {
        self.card = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`CreateCardsRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`card`](CreateCardsRequestBuilder::card)
    pub fn build(self) -> Result<CreateCardsRequest, BuildError> {
        Ok(CreateCardsRequest {
            card: self.card.ok_or_else(|| BuildError::missing_field("card"))?,
        })
    }
}
