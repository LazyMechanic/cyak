use serde::{Deserialize, Serialize};

use super::lang::Language;
use super::version::Version;

#[derive(Debug, Deserialize, Serialize)]
pub struct ProjectConfig {
    name: String,
    namespace: String,
    version: Version,
    language: Language,
    targets: Vec<Target>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Target {
    #[serde(rename = "type")]
    kind: TargetKind,
    name: String,
    version: Version,
    properties: Vec<TargetProperty>,
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
    key: String,
    value: String,
}
