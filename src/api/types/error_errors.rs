pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(untagged)]
pub enum ErrorErrors {
    StringList(Vec<String>),

    StringToStringListMap(HashMap<String, Vec<String>>),
}

impl ErrorErrors {
    pub fn is_string_list(&self) -> bool {
        matches!(self, Self::StringList(_))
    }

    pub fn is_string_to_string_list_map(&self) -> bool {
        matches!(self, Self::StringToStringListMap(_))
    }

    pub fn as_string_list(&self) -> Option<&Vec<String>> {
        match self {
            Self::StringList(value) => Some(value),
            _ => None,
        }
    }

    pub fn into_string_list(self) -> Option<Vec<String>> {
        match self {
            Self::StringList(value) => Some(value),
            _ => None,
        }
    }

    pub fn as_string_to_string_list_map(&self) -> Option<&HashMap<String, Vec<String>>> {
        match self {
            Self::StringToStringListMap(value) => Some(value),
            _ => None,
        }
    }

    pub fn into_string_to_string_list_map(self) -> Option<HashMap<String, Vec<String>>> {
        match self {
            Self::StringToStringListMap(value) => Some(value),
            _ => None,
        }
    }
}
