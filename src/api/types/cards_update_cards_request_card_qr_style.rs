pub use crate::prelude::*;

/// What the wallet pass QR holds. Changing it refreshes installed passes; passes still showing the old QR keep scanning
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum UpdateCardsRequestCardQrStyle {
    Link,
    Code,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for UpdateCardsRequestCardQrStyle {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Link => serializer.serialize_str("link"),
            Self::Code => serializer.serialize_str("code"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for UpdateCardsRequestCardQrStyle {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "link" => Ok(Self::Link),
            "code" => Ok(Self::Code),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for UpdateCardsRequestCardQrStyle {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Link => write!(f, "link"),
            Self::Code => write!(f, "code"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
