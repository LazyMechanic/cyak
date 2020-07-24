use serde::{Deserialize, Serialize};

use super::lang::Language;
use super::version::Version;

#[derive(Debug, Deserialize, Serialize)]
pub struct PresetConfig {
    pub name: String,
    pub version: String,
    pub author: String,
    pub description: String,
    pub default_values: PresetDefaultValues,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct PresetDefaultValues {
    pub language: Language,
    pub version: Version,
    pub git: bool,
    pub target_properties: TargetProperties,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct TargetProperties {
    pub custom: Vec<CustomProperty>,
    pub common: Vec<CommonProperty>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct CustomProperty {
    pub display: String,
    pub description: String,
    pub key: String,
    pub value_pattern: String,
    pub default: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct CommonProperty {
    pub key: String,
    pub value: String,
}
