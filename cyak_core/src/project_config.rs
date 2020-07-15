use serde::{Deserialize, Serialize};

use super::lang::Language;
use super::version::Version;

#[derive(Debug, Deserialize, Serialize)]
pub struct ProjectConfig {
    pub name: String,
    pub namespace: String,
    pub version: Version,
    pub language: Language,
    pub targets: Vec<Target>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Target {
    #[serde(rename = "type")]
    pub kind: TargetKind,
    pub name: String,
    pub version: Version,
    pub properties: Vec<TargetProperty>,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "lowercase", untagged)]
pub enum TargetKind {
    Executable,
    Library,
    Interface,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct TargetProperty {
    pub key: String,
    pub value: String,
}
