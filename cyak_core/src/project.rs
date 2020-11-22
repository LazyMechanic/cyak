use serde::{Deserialize, Serialize};

use super::preset::VariableStorage;
use super::PresetConfig;
use super::Version;

#[derive(Debug, Default, Deserialize, Serialize)]
pub struct ProjectConfig {
    pub name: String,
    pub version: Version,
    pub targets: Vec<Target>,
    pub variables: Vec<Variable>,
}

impl ProjectConfig {
    pub fn init_variables(&mut self, preset_config: &PresetConfig) {
        self.variables = preset_config
            .variables
            .iter()
            .filter(|v| v.storages.contains(&VariableStorage::Project))
            .map(|v| Variable {
                key: v.key.clone(),
                value: v.default.clone(),
            })
            .collect();
    }
}

#[derive(Debug, Default, Deserialize, Serialize)]
pub struct Target {
    pub kind: TargetKind,
    pub name: String,
    pub variables: Vec<Variable>,
}

impl Target {
    pub fn init_variables(&mut self, preset_config: &PresetConfig) {
        self.variables = preset_config
            .variables
            .iter()
            .filter(|v| {
                let storage = match &self.kind {
                    TargetKind::Executable => VariableStorage::Executable,
                    TargetKind::Library => VariableStorage::Library,
                    TargetKind::Interface => VariableStorage::Interface,
                    TargetKind::Test => VariableStorage::Test,
                };
                v.storages.contains(&storage)
            })
            .map(|v| Variable {
                key: v.key.clone(),
                value: v.default.clone(),
            })
            .collect();
    }
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
