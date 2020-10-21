use serde::{Deserialize, Serialize};

use super::version::Version;

#[derive(Debug, Default, Deserialize, Serialize)]
pub struct ProjectConfig {
    pub name: String,
    pub version: Version,
    pub targets: Vec<Target>,
    pub variables: Vec<Variable>,
}

#[derive(Debug, Default, Deserialize, Serialize)]
pub struct Target {
    pub kind: TargetKind,
    pub name: String,
    pub variables: Vec<Variable>,
}

#[derive(Debug, Deserialize, Serialize, Copy, Clone, Eq, PartialEq, Hash)]
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
pub struct Variable {
    pub key: String,
    pub value: String,
}
