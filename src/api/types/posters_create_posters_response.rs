pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct CreatePostersResponse {
    /// Parent store ID
    #[serde(default)]
    pub account_id: i64,
    /// Whether the public signup URL is live
    #[serde(default)]
    pub active: bool,
    /// Loyalty card customers are signed up to
    #[serde(default)]
    pub card_id: i64,
    /// ISO 8601 creation timestamp
    #[serde(default)]
    pub created_at: String,
    /// URL of the on screen version of the poster
    #[serde(default)]
    pub display_url: String,
    /// Unique poster ID
    #[serde(default)]
    pub id: i64,
    /// Paper size the poster is laid out for
    #[serde(default)]
    pub paper_size: String,
    /// Hex colour for the poster background
    #[serde(default)]
    pub primary_color: String,
    /// URL encoded in the QR code
    #[serde(default)]
    pub qr_code_url: String,
    /// Hex accent colour
    #[serde(default)]
    pub secondary_color: String,
    /// Public URL the QR code points at
    #[serde(default)]
    pub signup_url: String,
    /// Hex colour for poster text
    #[serde(default)]
    pub text_color: String,
    /// Heading printed on the poster
    #[serde(default)]
    pub title: String,
    /// ISO 8601 last-update timestamp
    #[serde(default)]
    pub updated_at: String,
}

impl CreatePostersResponse {
    pub fn builder() -> CreatePostersResponseBuilder {
        <CreatePostersResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CreatePostersResponseBuilder {
    account_id: Option<i64>,
    active: Option<bool>,
    card_id: Option<i64>,
    created_at: Option<String>,
    display_url: Option<String>,
    id: Option<i64>,
    paper_size: Option<String>,
    primary_color: Option<String>,
    qr_code_url: Option<String>,
    secondary_color: Option<String>,
    signup_url: Option<String>,
    text_color: Option<String>,
    title: Option<String>,
    updated_at: Option<String>,
}

impl CreatePostersResponseBuilder {
    pub fn account_id(mut self, value: i64) -> Self {
        self.account_id = Some(value);
        self
    }

    pub fn active(mut self, value: bool) -> Self {
        self.active = Some(value);
        self
    }

    pub fn card_id(mut self, value: i64) -> Self {
        self.card_id = Some(value);
        self
    }

    pub fn created_at(mut self, value: impl Into<String>) -> Self {
        self.created_at = Some(value.into());
        self
    }

    pub fn display_url(mut self, value: impl Into<String>) -> Self {
        self.display_url = Some(value.into());
        self
    }

    pub fn id(mut self, value: i64) -> Self {
        self.id = Some(value);
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

    pub fn qr_code_url(mut self, value: impl Into<String>) -> Self {
        self.qr_code_url = Some(value.into());
        self
    }

    pub fn secondary_color(mut self, value: impl Into<String>) -> Self {
        self.secondary_color = Some(value.into());
        self
    }

    pub fn signup_url(mut self, value: impl Into<String>) -> Self {
        self.signup_url = Some(value.into());
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

    pub fn updated_at(mut self, value: impl Into<String>) -> Self {
        self.updated_at = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`CreatePostersResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`account_id`](CreatePostersResponseBuilder::account_id)
    /// - [`active`](CreatePostersResponseBuilder::active)
    /// - [`card_id`](CreatePostersResponseBuilder::card_id)
    /// - [`created_at`](CreatePostersResponseBuilder::created_at)
    /// - [`display_url`](CreatePostersResponseBuilder::display_url)
    /// - [`id`](CreatePostersResponseBuilder::id)
    /// - [`paper_size`](CreatePostersResponseBuilder::paper_size)
    /// - [`primary_color`](CreatePostersResponseBuilder::primary_color)
    /// - [`qr_code_url`](CreatePostersResponseBuilder::qr_code_url)
    /// - [`secondary_color`](CreatePostersResponseBuilder::secondary_color)
    /// - [`signup_url`](CreatePostersResponseBuilder::signup_url)
    /// - [`text_color`](CreatePostersResponseBuilder::text_color)
    /// - [`title`](CreatePostersResponseBuilder::title)
    /// - [`updated_at`](CreatePostersResponseBuilder::updated_at)
    pub fn build(self) -> Result<CreatePostersResponse, BuildError> {
        Ok(CreatePostersResponse {
            account_id: self
                .account_id
                .ok_or_else(|| BuildError::missing_field("account_id"))?,
            active: self
                .active
                .ok_or_else(|| BuildError::missing_field("active"))?,
            card_id: self
                .card_id
                .ok_or_else(|| BuildError::missing_field("card_id"))?,
            created_at: self
                .created_at
                .ok_or_else(|| BuildError::missing_field("created_at"))?,
            display_url: self
                .display_url
                .ok_or_else(|| BuildError::missing_field("display_url"))?,
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            paper_size: self
                .paper_size
                .ok_or_else(|| BuildError::missing_field("paper_size"))?,
            primary_color: self
                .primary_color
                .ok_or_else(|| BuildError::missing_field("primary_color"))?,
            qr_code_url: self
                .qr_code_url
                .ok_or_else(|| BuildError::missing_field("qr_code_url"))?,
            secondary_color: self
                .secondary_color
                .ok_or_else(|| BuildError::missing_field("secondary_color"))?,
            signup_url: self
                .signup_url
                .ok_or_else(|| BuildError::missing_field("signup_url"))?,
            text_color: self
                .text_color
                .ok_or_else(|| BuildError::missing_field("text_color"))?,
            title: self
                .title
                .ok_or_else(|| BuildError::missing_field("title"))?,
            updated_at: self
                .updated_at
                .ok_or_else(|| BuildError::missing_field("updated_at"))?,
        })
    }
}
