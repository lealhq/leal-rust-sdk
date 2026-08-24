pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct CreatePostersRequestPoster {
    /// Whether the poster is active (defaults to true)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub active: Option<bool>,
    /// ID of the loyalty card this poster links to
    #[serde(default)]
    pub card_id: i64,
    /// Print size – one of: a4, a5, a6, letter
    #[serde(skip_serializing_if = "Option::is_none")]
    pub paper_size: Option<String>,
    /// Primary brand color as a hex string (e.g. '#FF5733')
    #[serde(skip_serializing_if = "Option::is_none")]
    pub primary_color: Option<String>,
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
    paper_size: Option<String>,
    primary_color: Option<String>,
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

    pub fn paper_size(mut self, value: impl Into<String>) -> Self {
        self.paper_size = Some(value.into());
        self
    }

    pub fn primary_color(mut self, value: impl Into<String>) -> Self {
        self.primary_color = Some(value.into());
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
            paper_size: self.paper_size,
            primary_color: self.primary_color,
            secondary_color: self.secondary_color,
            text_color: self.text_color,
            title: self.title,
        })
    }
}
