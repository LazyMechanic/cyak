use serde::{Deserialize, Serialize};
use std::collections::HashSet;

#[derive(Debug, Deserialize, Serialize)]
pub struct PresetConfig {
    pub name: String,
    pub version: String,
    pub author: String,
    pub description: String,
    pub variables: Vec<PresetVariable>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct PresetVariable {
    pub display: String,
    pub description: String,
    pub key: String,
    pub value_pattern: String,
    pub default: String,
    pub storages: HashSet<VariableStorage>,
}

#[derive(Debug, Deserialize, Serialize, Copy, Clone, Eq, PartialEq, Hash)]
#[serde(rename_all = "lowercase")]
pub enum VariableStorage {
    Executable,
    Library,
    Interface,
    Test,
    Project,
}
