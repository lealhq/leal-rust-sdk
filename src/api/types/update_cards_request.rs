pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct UpdateCardsRequest {
    #[serde(default)]
    pub card: UpdateCardsRequestCard,
}

impl UpdateCardsRequest {
    pub fn builder() -> UpdateCardsRequestBuilder {
        <UpdateCardsRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct UpdateCardsRequestBuilder {
    card: Option<UpdateCardsRequestCard>,
}

impl UpdateCardsRequestBuilder {
    pub fn card(mut self, value: UpdateCardsRequestCard) -> Self {
        self.card = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`UpdateCardsRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`card`](UpdateCardsRequestBuilder::card)
    pub fn build(self) -> Result<UpdateCardsRequest, BuildError> {
        Ok(UpdateCardsRequest {
            card: self.card.ok_or_else(|| BuildError::missing_field("card"))?,
        })
    }
}
