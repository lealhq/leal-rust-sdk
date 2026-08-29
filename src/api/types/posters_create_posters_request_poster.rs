pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct CreatePostersRequestPoster {
    /// Whether the poster is active (defaults to true)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub active: Option<bool>,
    /// ID of the loyalty card this poster links to
    #[serde(default)]
    pub card_id: i64,
    /// Which contact fields appear on the public signup form
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contact_collection_mode: Option<String>,
    /// Minimum customer age required for signup. Requires require_birthday to be true.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub minimum_age: Option<f64>,
    /// Print size – one of: a4, a5, a6, letter
    #[serde(skip_serializing_if = "Option::is_none")]
    pub paper_size: Option<String>,
    /// Primary brand color as a hex string (e.g. '#FF5733')
    #[serde(skip_serializing_if = "Option::is_none")]
    pub primary_color: Option<String>,
    /// Whether date of birth is required on the public signup form
    #[serde(skip_serializing_if = "Option::is_none")]
    pub require_birthday: Option<bool>,
    /// Whether email is required when it is collected
    #[serde(skip_serializing_if = "Option::is_none")]
    pub require_email: Option<bool>,
    /// Whether phone number is required when it is collected
    #[serde(skip_serializing_if = "Option::is_none")]
    pub require_phone: Option<bool>,
    /// Secondary brand color as a hex string
    #[serde(skip_serializing_if = "Option::is_none")]
    pub secondary_color: Option<String>,
    /// Text color as a hex string
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text_color: Option<String>,
    /// Headline text displayed on the poster
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
}

impl CreatePostersRequestPoster {
    pub fn builder() -> CreatePostersRequestPosterBuilder {
        <CreatePostersRequestPosterBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CreatePostersRequestPosterBuilder {
    active: Option<bool>,
    card_id: Option<i64>,
    contact_collection_mode: Option<String>,
    minimum_age: Option<f64>,
    paper_size: Option<String>,
    primary_color: Option<String>,
    require_birthday: Option<bool>,
    require_email: Option<bool>,
    require_phone: Option<bool>,
    secondary_color: Option<String>,
    text_color: Option<String>,
    title: Option<String>,
}

impl CreatePostersRequestPosterBuilder {
    pub fn active(mut self, value: bool) -> Self {
        self.active = Some(value);
        self
    }

    pub fn card_id(mut self, value: i64) -> Self {
        self.card_id = Some(value);
        self
    }

    pub fn contact_collection_mode(mut self, value: impl Into<String>) -> Self {
        self.contact_collection_mode = Some(value.into());
        self
    }

    pub fn minimum_age(mut self, value: f64) -> Self {
        self.minimum_age = Some(value);
        self
    }

    pub fn paper_size(mut self, value: impl Into<String>) -> Self {
        self.paper_size = Some(value.into());
        self
    }

    pub fn primary_color(mut self, value: impl Into<String>) -> Self {
        self.primary_color = Some(value.into());
        self
    }

    pub fn require_birthday(mut self, value: bool) -> Self {
        self.require_birthday = Some(value);
        self
    }

    pub fn require_email(mut self, value: bool) -> Self {
        self.require_email = Some(value);
        self
    }

    pub fn require_phone(mut self, value: bool) -> Self {
        self.require_phone = Some(value);
        self
    }

    pub fn secondary_color(mut self, value: impl Into<String>) -> Self {
        self.secondary_color = Some(value.into());
        self
    }

    pub fn text_color(mut self, value: impl Into<String>) -> Self {
        self.text_color = Some(value.into());
        self
    }

    pub fn title(mut self, value: impl Into<String>) -> Self {
        self.title = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`CreatePostersRequestPoster`].
    /// This method will fail if any of the following fields are not set:
    /// - [`card_id`](CreatePostersRequestPosterBuilder::card_id)
    pub fn build(self) -> Result<CreatePostersRequestPoster, BuildError> {
        Ok(CreatePostersRequestPoster {
            active: self.active,
            card_id: self
                .card_id
                .ok_or_else(|| BuildError::missing_field("card_id"))?,
            contact_collection_mode: self.contact_collection_mode,
            minimum_age: self.minimum_age,
            paper_size: self.paper_size,
            primary_color: self.primary_color,
            require_birthday: self.require_birthday,
            require_email: self.require_email,
            require_phone: self.require_phone,
            secondary_color: self.secondary_color,
            text_color: self.text_color,
            title: self.title,
        })
    }
}
