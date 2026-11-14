pub use crate::prelude::*;

/// How the logo sits on Google Wallet. Defaults to 'round'
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum UpdateCardsRequestCardGoogleLogoStyle {
    Round,
    Wide,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for UpdateCardsRequestCardGoogleLogoStyle {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Round => serializer.serialize_str("round"),
            Self::Wide => serializer.serialize_str("wide"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for UpdateCardsRequestCardGoogleLogoStyle {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "round" => Ok(Self::Round),
            "wide" => Ok(Self::Wide),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for UpdateCardsRequestCardGoogleLogoStyle {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Round => write!(f, "round"),
            Self::Wide => write!(f, "wide"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
