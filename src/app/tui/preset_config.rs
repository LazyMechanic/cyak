use serde::{Deserialize, Serialize};
use std::cmp::Ordering;

#[derive(Debug, Deserialize, Serialize)]
pub struct PresetConfig {
    pub name:           String,
    pub version:        String,
    pub author:         String,
    pub description:    String,
    pub default_values: PresetDefaultValues,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct PresetDefaultValues {
    pub language:          Language,
    pub version:           Version,
    pub git:               bool,
    pub target_properties: TargetProperties,
}

#[derive(Debug, Deserialize, Serialize, Copy, Clone, Eq, PartialEq)]
pub enum Language {
    #[serde(rename = "C++")]
    Cpp,
    C,
}

#[derive(Debug, Deserialize, Serialize, Eq, PartialEq)]
pub struct Version {
    pub major: u32,
    pub minor: u32,
    pub patch: u32,
}

impl PartialOrd for Version {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Version {
    fn cmp(&self, other: &Self) -> Ordering {
        // Compare major
        return if self.major > other.major {
            Ordering::Greater
        } else if self.major == other.major {
            // Compare minor
            if self.minor > other.minor {
                Ordering::Greater
            } else if self.minor == other.minor {
                // Compare patch
                if self.patch > other.patch {
                    Ordering::Greater
                } else if self.patch == other.patch {
                    Ordering::Equal
                } else {
                    Ordering::Less
                }
            } else {
                Ordering::Less
            }
        } else {
            Ordering::Less
        };
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct TargetProperties {
    custom: Vec<CustomProperty>,
    common: Vec<CommonProperty>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct CustomProperty {
    display:       String,
    description:   String,
    key:           String,
    value_pattern: String,
    default:       String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct CommonProperty {
    key:   String,
    value: String,
}
