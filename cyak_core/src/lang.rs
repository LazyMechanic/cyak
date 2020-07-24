use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Copy, Clone, Eq, PartialEq)]
pub enum Language {
    #[serde(rename = "CXX")]
    Cpp,
    C,
}
