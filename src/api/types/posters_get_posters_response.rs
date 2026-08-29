pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct GetPostersResponse {
    /// Parent store ID
    #[serde(default)]
    pub account_id: i64,
    /// Whether the public signup URL is live
    #[serde(default)]
    pub active: bool,
    /// Loyalty card customers are signed up to
    #[serde(default)]
    pub card_id: i64,
    /// Whether the public signup form collects email
    #[serde(default)]
    pub collect_email: bool,
    /// Whether the public signup form collects phone number
    #[serde(default)]
    pub collect_phone: bool,
    /// Which contact fields appear on the public signup form: 'email_and_phone', 'email_only', or 'phone_only'
    #[serde(default)]
    pub contact_collection_mode: String,
    /// ISO 8601 creation timestamp
    #[serde(default)]
    pub created_at: String,
    /// URL of the on screen version of the poster
    #[serde(default)]
    pub display_url: String,
    /// Unique poster ID
    #[serde(default)]
    pub id: i64,
    /// Minimum customer age required for signup
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers")]
    pub minimum_age: f64,
    /// Paper size the poster is laid out for
    #[serde(default)]
    pub paper_size: String,
    /// Hex colour for the poster background
    #[serde(default)]
    pub primary_color: String,
    /// URL encoded in the QR code
    #[serde(default)]
    pub qr_code_url: String,
    /// Whether date of birth is required on the public signup form
    #[serde(default)]
    pub require_birthday: bool,
    /// Whether email is required when it is collected
    #[serde(default)]
    pub require_email: bool,
    /// Whether phone number is required when it is collected
    #[serde(default)]
    pub require_phone: bool,
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

impl GetPostersResponse {
    pub fn builder() -> GetPostersResponseBuilder {
        <GetPostersResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct GetPostersResponseBuilder {
    account_id: Option<i64>,
    active: Option<bool>,
    card_id: Option<i64>,
    collect_email: Option<bool>,
    collect_phone: Option<bool>,
    contact_collection_mode: Option<String>,
    created_at: Option<String>,
    display_url: Option<String>,
    id: Option<i64>,
    minimum_age: Option<f64>,
    paper_size: Option<String>,
    primary_color: Option<String>,
    qr_code_url: Option<String>,
    require_birthday: Option<bool>,
    require_email: Option<bool>,
    require_phone: Option<bool>,
    secondary_color: Option<String>,
    signup_url: Option<String>,
    text_color: Option<String>,
    title: Option<String>,
    updated_at: Option<String>,
}

impl GetPostersResponseBuilder {
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

    pub fn collect_email(mut self, value: bool) -> Self {
        self.collect_email = Some(value);
        self
    }

    pub fn collect_phone(mut self, value: bool) -> Self {
        self.collect_phone = Some(value);
        self
    }

    pub fn contact_collection_mode(mut self, value: impl Into<String>) -> Self {
        self.contact_collection_mode = Some(value.into());
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

    pub fn qr_code_url(mut self, value: impl Into<String>) -> Self {
        self.qr_code_url = Some(value.into());
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

    /// Consumes the builder and constructs a [`GetPostersResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`account_id`](GetPostersResponseBuilder::account_id)
    /// - [`active`](GetPostersResponseBuilder::active)
    /// - [`card_id`](GetPostersResponseBuilder::card_id)
    /// - [`collect_email`](GetPostersResponseBuilder::collect_email)
    /// - [`collect_phone`](GetPostersResponseBuilder::collect_phone)
    /// - [`contact_collection_mode`](GetPostersResponseBuilder::contact_collection_mode)
    /// - [`created_at`](GetPostersResponseBuilder::created_at)
    /// - [`display_url`](GetPostersResponseBuilder::display_url)
    /// - [`id`](GetPostersResponseBuilder::id)
    /// - [`minimum_age`](GetPostersResponseBuilder::minimum_age)
    /// - [`paper_size`](GetPostersResponseBuilder::paper_size)
    /// - [`primary_color`](GetPostersResponseBuilder::primary_color)
    /// - [`qr_code_url`](GetPostersResponseBuilder::qr_code_url)
    /// - [`require_birthday`](GetPostersResponseBuilder::require_birthday)
    /// - [`require_email`](GetPostersResponseBuilder::require_email)
    /// - [`require_phone`](GetPostersResponseBuilder::require_phone)
    /// - [`secondary_color`](GetPostersResponseBuilder::secondary_color)
    /// - [`signup_url`](GetPostersResponseBuilder::signup_url)
    /// - [`text_color`](GetPostersResponseBuilder::text_color)
    /// - [`title`](GetPostersResponseBuilder::title)
    /// - [`updated_at`](GetPostersResponseBuilder::updated_at)
    pub fn build(self) -> Result<GetPostersResponse, BuildError> {
        Ok(GetPostersResponse {
            account_id: self
                .account_id
                .ok_or_else(|| BuildError::missing_field("account_id"))?,
            active: self
                .active
                .ok_or_else(|| BuildError::missing_field("active"))?,
            card_id: self
                .card_id
                .ok_or_else(|| BuildError::missing_field("card_id"))?,
            collect_email: self
                .collect_email
                .ok_or_else(|| BuildError::missing_field("collect_email"))?,
            collect_phone: self
                .collect_phone
                .ok_or_else(|| BuildError::missing_field("collect_phone"))?,
            contact_collection_mode: self
                .contact_collection_mode
                .ok_or_else(|| BuildError::missing_field("contact_collection_mode"))?,
            created_at: self
                .created_at
                .ok_or_else(|| BuildError::missing_field("created_at"))?,
            display_url: self
                .display_url
                .ok_or_else(|| BuildError::missing_field("display_url"))?,
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            minimum_age: self
                .minimum_age
                .ok_or_else(|| BuildError::missing_field("minimum_age"))?,
            paper_size: self
                .paper_size
                .ok_or_else(|| BuildError::missing_field("paper_size"))?,
            primary_color: self
                .primary_color
                .ok_or_else(|| BuildError::missing_field("primary_color"))?,
            qr_code_url: self
                .qr_code_url
                .ok_or_else(|| BuildError::missing_field("qr_code_url"))?,
            require_birthday: self
                .require_birthday
                .ok_or_else(|| BuildError::missing_field("require_birthday"))?,
            require_email: self
                .require_email
                .ok_or_else(|| BuildError::missing_field("require_email"))?,
            require_phone: self
                .require_phone
                .ok_or_else(|| BuildError::missing_field("require_phone"))?,
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
