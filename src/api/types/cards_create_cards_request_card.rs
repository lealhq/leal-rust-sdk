pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct CreateCardsRequestCard {
    /// Up to two extra front-of-pass fields. Blank values are ignored.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auxiliary_fields: Option<Vec<String>>,
    /// Hex colour for the card background (e.g. '#6B4226')
    #[serde(skip_serializing_if = "Option::is_none")]
    pub card_color: Option<String>,
    /// Card expiry timestamp (ISO 8601)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expires_at: Option<String>,
    /// Optional header text displayed on the card
    #[serde(skip_serializing_if = "Option::is_none")]
    pub header_text: Option<String>,
    /// Number of stamps pre-filled on new customer cards (must be >= 0 and < stamps_required)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub initial_stamps: Option<i64>,
    /// Card name (e.g. 'Coffee Loyalty Card')
    #[serde(default)]
    pub name: String,
    /// Whether wallet passes show the member name field
    #[serde(skip_serializing_if = "Option::is_none")]
    pub show_member_field: Option<bool>,
    /// Whether wallet passes show the stamps-to-reward field
    #[serde(skip_serializing_if = "Option::is_none")]
    pub show_stamps_to_reward_field: Option<bool>,
    /// Hex colour for stamp backgrounds
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stamp_background_color: Option<String>,
    /// Hex colour for stamp icons
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stamp_color: Option<String>,
    /// Stamp icon identifier
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stamp_icon: Option<String>,
    /// Number of stamps needed to complete the card (1–21)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stamps_required: Option<i64>,
    /// Hex colour for the strip (used when strip_type is 'color')
    #[serde(skip_serializing_if = "Option::is_none")]
    pub strip_color: Option<String>,
    /// Preset strip image identifier (used when strip_type is 'preset')
    #[serde(skip_serializing_if = "Option::is_none")]
    pub strip_preset: Option<String>,
    /// Strip image type
    #[serde(skip_serializing_if = "Option::is_none")]
    pub strip_type: Option<String>,
    /// Hex colour for card text
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text_color: Option<String>,
}

impl CreateCardsRequestCard {
    pub fn builder() -> CreateCardsRequestCardBuilder {
        <CreateCardsRequestCardBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CreateCardsRequestCardBuilder {
    auxiliary_fields: Option<Vec<String>>,
    card_color: Option<String>,
    expires_at: Option<String>,
    header_text: Option<String>,
    initial_stamps: Option<i64>,
    name: Option<String>,
    show_member_field: Option<bool>,
    show_stamps_to_reward_field: Option<bool>,
    stamp_background_color: Option<String>,
    stamp_color: Option<String>,
    stamp_icon: Option<String>,
    stamps_required: Option<i64>,
    strip_color: Option<String>,
    strip_preset: Option<String>,
    strip_type: Option<String>,
    text_color: Option<String>,
}

impl CreateCardsRequestCardBuilder {
    pub fn auxiliary_fields(mut self, value: Vec<String>) -> Self {
        self.auxiliary_fields = Some(value);
        self
    }

    pub fn card_color(mut self, value: impl Into<String>) -> Self {
        self.card_color = Some(value.into());
        self
    }

    pub fn expires_at(mut self, value: impl Into<String>) -> Self {
        self.expires_at = Some(value.into());
        self
    }

    pub fn header_text(mut self, value: impl Into<String>) -> Self {
        self.header_text = Some(value.into());
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

    pub fn show_member_field(mut self, value: bool) -> Self {
        self.show_member_field = Some(value);
        self
    }

    pub fn show_stamps_to_reward_field(mut self, value: bool) -> Self {
        self.show_stamps_to_reward_field = Some(value);
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

    /// Consumes the builder and constructs a [`CreateCardsRequestCard`].
    /// This method will fail if any of the following fields are not set:
    /// - [`name`](CreateCardsRequestCardBuilder::name)
    pub fn build(self) -> Result<CreateCardsRequestCard, BuildError> {
        Ok(CreateCardsRequestCard {
            auxiliary_fields: self.auxiliary_fields,
            card_color: self.card_color,
            expires_at: self.expires_at,
            header_text: self.header_text,
            initial_stamps: self.initial_stamps,
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
            show_member_field: self.show_member_field,
            show_stamps_to_reward_field: self.show_stamps_to_reward_field,
            stamp_background_color: self.stamp_background_color,
            stamp_color: self.stamp_color,
            stamp_icon: self.stamp_icon,
            stamps_required: self.stamps_required,
            strip_color: self.strip_color,
            strip_preset: self.strip_preset,
            strip_type: self.strip_type,
            text_color: self.text_color,
        })
    }
}
