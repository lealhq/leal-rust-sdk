pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct UpdatePostersRequestPoster {
    /// Whether the poster is active
    #[serde(skip_serializing_if = "Option::is_none")]
    pub active: Option<bool>,
    /// Print size – one of: a4, a5, a6, letter
    #[serde(skip_serializing_if = "Option::is_none")]
    pub paper_size: Option<String>,
    /// Primary brand color as a hex string
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

impl UpdatePostersRequestPoster {
    pub fn builder() -> UpdatePostersRequestPosterBuilder {
        <UpdatePostersRequestPosterBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct UpdatePostersRequestPosterBuilder {
    active: Option<bool>,
    paper_size: Option<String>,
    primary_color: Option<String>,
    secondary_color: Option<String>,
    text_color: Option<String>,
    title: Option<String>,
}

impl UpdatePostersRequestPosterBuilder {
    pub fn active(mut self, value: bool) -> Self {
        self.active = Some(value);
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

    /// Consumes the builder and constructs a [`UpdatePostersRequestPoster`].
    pub fn build(self) -> Result<UpdatePostersRequestPoster, BuildError> {
        Ok(UpdatePostersRequestPoster {
            active: self.active,
            paper_size: self.paper_size,
            primary_color: self.primary_color,
            secondary_color: self.secondary_color,
            text_color: self.text_color,
            title: self.title,
        })
    }
}
