use serde::{Deserialize, Serialize};

use super::lang::Language;
use super::version::Version;
use super::PresetConfig;

#[derive(Debug, Default, Deserialize, Serialize)]
pub struct ProjectConfig {
    pub name: String,
    pub namespace: String,
    pub version: Version,
    pub language: Language,
    pub targets: Vec<Target>,
}

#[derive(Debug, Default, Deserialize, Serialize)]
pub struct Target {
    pub kind: TargetKind,
    pub name: String,
    pub version: Version,
    pub properties: Vec<TargetProperty>,
}

#[derive(Debug, Deserialize, Serialize, Copy, Clone, Eq, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum TargetKind {
    Executable,
    Library,
    Interface,
    Test,
}

impl Default for TargetKind {
    fn default() -> Self {
        TargetKind::Executable
    }
}

#[derive(Debug, Default, Deserialize, Serialize)]
pub struct TargetProperty {
    pub key: String,
    pub value: String,
}