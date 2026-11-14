pub use crate::prelude::*;

/// Apple Wallet layout. 'poster' is iOS 27's full-bleed layout; older iPhones show the store card. Defaults to 'store_card'
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum UpdateCardsRequestCardApplePassStyle {
    StoreCard,
    Poster,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for UpdateCardsRequestCardApplePassStyle {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::StoreCard => serializer.serialize_str("store_card"),
            Self::Poster => serializer.serialize_str("poster"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for UpdateCardsRequestCardApplePassStyle {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "store_card" => Ok(Self::StoreCard),
            "poster" => Ok(Self::Poster),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for UpdateCardsRequestCardApplePassStyle {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::StoreCard => write!(f, "store_card"),
            Self::Poster => write!(f, "poster"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
