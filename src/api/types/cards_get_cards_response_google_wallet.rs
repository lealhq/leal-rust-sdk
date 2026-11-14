pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct GetCardsResponseGoogleWallet {
    /// What fills the hero image behind the stamps: 'hero_image' (uploaded in Leal), 'strip_image', 'preset' or 'color'
    #[serde(default)]
    pub hero_image: String,
    /// The text colour Google Wallet uses on this card: it picks black or white itself and ignores text_color
    #[serde(default)]
    pub text_color: String,
    /// Guidelines the card doesn't meet: any of 'color', 'title', 'issuer_name', 'field_labels', 'field_values', 'logo', 'hero'. Empty when it meets them all
    #[serde(default)]
    pub warnings: Vec<String>,
}

impl GetCardsResponseGoogleWallet {
    pub fn builder() -> GetCardsResponseGoogleWalletBuilder {
        <GetCardsResponseGoogleWalletBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct GetCardsResponseGoogleWalletBuilder {
    hero_image: Option<String>,
    text_color: Option<String>,
    warnings: Option<Vec<String>>,
}

impl GetCardsResponseGoogleWalletBuilder {
    pub fn hero_image(mut self, value: impl Into<String>) -> Self {
        self.hero_image = Some(value.into());
        self
    }

    pub fn text_color(mut self, value: impl Into<String>) -> Self {
        self.text_color = Some(value.into());
        self
    }

    pub fn warnings(mut self, value: Vec<String>) -> Self {
        self.warnings = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`GetCardsResponseGoogleWallet`].
    /// This method will fail if any of the following fields are not set:
    /// - [`hero_image`](GetCardsResponseGoogleWalletBuilder::hero_image)
    /// - [`text_color`](GetCardsResponseGoogleWalletBuilder::text_color)
    /// - [`warnings`](GetCardsResponseGoogleWalletBuilder::warnings)
    pub fn build(self) -> Result<GetCardsResponseGoogleWallet, BuildError> {
        Ok(GetCardsResponseGoogleWallet {
            hero_image: self
                .hero_image
                .ok_or_else(|| BuildError::missing_field("hero_image"))?,
            text_color: self
                .text_color
                .ok_or_else(|| BuildError::missing_field("text_color"))?,
            warnings: self
                .warnings
                .ok_or_else(|| BuildError::missing_field("warnings"))?,
        })
    }
}
