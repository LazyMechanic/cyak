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
    custom: Vec<CustomProperty>,
    common: Vec<CommonProperty>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct CustomProperty {
    display: String,
    description: String,
    key: String,
    value_pattern: String,
    default: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct CommonProperty {
    key: String,
    value: String,
}
