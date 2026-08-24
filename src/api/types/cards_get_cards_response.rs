pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct GetCardsResponse {
    /// ISO 8601 timestamp when the card was archived, or null if active
    #[serde(default)]
    pub archived_at: String,
    /// Hex colour for the card background (e.g. '#6B4226')
    #[serde(default)]
    pub card_color: String,
    /// ISO 8601 creation timestamp
    #[serde(default)]
    pub created_at: String,
    /// Number of customer card instances issued
    #[serde(default)]
    pub customer_cards_count: i64,
    /// Optional header text displayed on the card
    #[serde(default)]
    pub header_text: String,
    /// Unique card ID
    #[serde(default)]
    pub id: i64,
    /// Number of stamps pre-filled on new customer cards (0 to stamps_required - 1)
    #[serde(default)]
    pub initial_stamps: i64,
    /// Card name (e.g. 'Coffee Loyalty Card')
    #[serde(default)]
    pub name: String,
    /// Number of rewards defined for this card
    #[serde(default)]
    pub rewards_count: i64,
    /// Hex colour for stamp backgrounds
    #[serde(default)]
    pub stamp_background_color: String,
    /// Hex colour for stamp icons
    #[serde(default)]
    pub stamp_color: String,
    /// Icon used for stamps (e.g. 'coffee', 'heart', 'star')
    #[serde(default)]
    pub stamp_icon: String,
    /// Number of stamps needed to complete the card (1–21)
    #[serde(default)]
    pub stamps_required: i64,
    /// Hex colour for the strip (when strip_type is 'color')
    #[serde(default)]
    pub strip_color: String,
    /// Preset strip image identifier (when strip_type is 'preset')
    #[serde(default)]
    pub strip_preset: String,
    /// Strip image type: 'color', 'image', or 'preset'
    #[serde(default)]
    pub strip_type: String,
    /// Hex colour for card text
    #[serde(default)]
    pub text_color: String,
    /// ISO 8601 last-update timestamp
    #[serde(default)]
    pub updated_at: String,
}

impl GetCardsResponse {
    pub fn builder() -> GetCardsResponseBuilder {
        <GetCardsResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct GetCardsResponseBuilder {
    archived_at: Option<String>,
    card_color: Option<String>,
    created_at: Option<String>,
    customer_cards_count: Option<i64>,
    header_text: Option<String>,
    id: Option<i64>,
    initial_stamps: Option<i64>,
    name: Option<String>,
    rewards_count: Option<i64>,
    stamp_background_color: Option<String>,
    stamp_color: Option<String>,
    stamp_icon: Option<String>,
    stamps_required: Option<i64>,
    strip_color: Option<String>,
    strip_preset: Option<String>,
    strip_type: Option<String>,
    text_color: Option<String>,
    updated_at: Option<String>,
}

impl GetCardsResponseBuilder {
    pub fn archived_at(mut self, value: impl Into<String>) -> Self {
        self.archived_at = Some(value.into());
        self
    }

    pub fn card_color(mut self, value: impl Into<String>) -> Self {
        self.card_color = Some(value.into());
        self
    }

    pub fn created_at(mut self, value: impl Into<String>) -> Self {
        self.created_at = Some(value.into());
        self
    }

    pub fn customer_cards_count(mut self, value: i64) -> Self {
        self.customer_cards_count = Some(value);
        self
    }

    pub fn header_text(mut self, value: impl Into<String>) -> Self {
        self.header_text = Some(value.into());
        self
    }

    pub fn id(mut self, value: i64) -> Self {
        self.id = Some(value);
        self
    }

    pub fn initial_stamps(mut self, value: i64) -> Self {
        self.initial_stamps = Some(value);
        self
    }

    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    pub fn rewards_count(mut self, value: i64) -> Self {
        self.rewards_count = Some(value);
        self
    }

    pub fn stamp_background_color(mut self, value: impl Into<String>) -> Self {
        self.stamp_background_color = Some(value.into());
        self
    }

    pub fn stamp_color(mut self, value: impl Into<String>) -> Self {
        self.stamp_color = Some(value.into());
        self
    }

    pub fn stamp_icon(mut self, value: impl Into<String>) -> Self {
        self.stamp_icon = Some(value.into());
        self
    }

    pub fn stamps_required(mut self, value: i64) -> Self {
        self.stamps_required = Some(value);
        self
    }

    pub fn strip_color(mut self, value: impl Into<String>) -> Self {
        self.strip_color = Some(value.into());
        self
    }

    pub fn strip_preset(mut self, value: impl Into<String>) -> Self {
        self.strip_preset = Some(value.into());
        self
    }

    pub fn strip_type(mut self, value: impl Into<String>) -> Self {
        self.strip_type = Some(value.into());
        self
    }

    pub fn text_color(mut self, value: impl Into<String>) -> Self {
        self.text_color = Some(value.into());
        self
    }

    pub fn updated_at(mut self, value: impl Into<String>) -> Self {
        self.updated_at = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`GetCardsResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`archived_at`](GetCardsResponseBuilder::archived_at)
    /// - [`card_color`](GetCardsResponseBuilder::card_color)
    /// - [`created_at`](GetCardsResponseBuilder::created_at)
    /// - [`customer_cards_count`](GetCardsResponseBuilder::customer_cards_count)
    /// - [`header_text`](GetCardsResponseBuilder::header_text)
    /// - [`id`](GetCardsResponseBuilder::id)
    /// - [`initial_stamps`](GetCardsResponseBuilder::initial_stamps)
    /// - [`name`](GetCardsResponseBuilder::name)
    /// - [`rewards_count`](GetCardsResponseBuilder::rewards_count)
    /// - [`stamp_background_color`](GetCardsResponseBuilder::stamp_background_color)
    /// - [`stamp_color`](GetCardsResponseBuilder::stamp_color)
    /// - [`stamp_icon`](GetCardsResponseBuilder::stamp_icon)
    /// - [`stamps_required`](GetCardsResponseBuilder::stamps_required)
    /// - [`strip_color`](GetCardsResponseBuilder::strip_color)
    /// - [`strip_preset`](GetCardsResponseBuilder::strip_preset)
    /// - [`strip_type`](GetCardsResponseBuilder::strip_type)
    /// - [`text_color`](GetCardsResponseBuilder::text_color)
    /// - [`updated_at`](GetCardsResponseBuilder::updated_at)
    pub fn build(self) -> Result<GetCardsResponse, BuildError> {
        Ok(GetCardsResponse {
            archived_at: self
                .archived_at
                .ok_or_else(|| BuildError::missing_field("archived_at"))?,
            card_color: self
                .card_color
                .ok_or_else(|| BuildError::missing_field("card_color"))?,
            created_at: self
                .created_at
                .ok_or_else(|| BuildError::missing_field("created_at"))?,
            customer_cards_count: self
                .customer_cards_count
                .ok_or_else(|| BuildError::missing_field("customer_cards_count"))?,
            header_text: self
                .header_text
                .ok_or_else(|| BuildError::missing_field("header_text"))?,
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            initial_stamps: self
                .initial_stamps
                .ok_or_else(|| BuildError::missing_field("initial_stamps"))?,
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
            rewards_count: self
                .rewards_count
                .ok_or_else(|| BuildError::missing_field("rewards_count"))?,
            stamp_background_color: self
                .stamp_background_color
                .ok_or_else(|| BuildError::missing_field("stamp_background_color"))?,
            stamp_color: self
                .stamp_color
                .ok_or_else(|| BuildError::missing_field("stamp_color"))?,
            stamp_icon: self
                .stamp_icon
                .ok_or_else(|| BuildError::missing_field("stamp_icon"))?,
            stamps_required: self
                .stamps_required
                .ok_or_else(|| BuildError::missing_field("stamps_required"))?,
            strip_color: self
                .strip_color
                .ok_or_else(|| BuildError::missing_field("strip_color"))?,
            strip_preset: self
                .strip_preset
                .ok_or_else(|| BuildError::missing_field("strip_preset"))?,
            strip_type: self
                .strip_type
                .ok_or_else(|| BuildError::missing_field("strip_type"))?,
            text_color: self
                .text_color
                .ok_or_else(|| BuildError::missing_field("text_color"))?,
            updated_at: self
                .updated_at
                .ok_or_else(|| BuildError::missing_field("updated_at"))?,
        })
    }
}
