use std::path::PathBuf;

use cyak_core::PresetConfig;
use cyak_core::ProjectConfig;

use crate::app::Error;
use crate::cli::Cli;

#[derive(Debug)]
pub struct Config {
    pub share_dir: PathBuf,
    pub project_dir: PathBuf,
}

impl Config {
    pub fn from_cli(cli: Cli) -> Self {
        Self {
            share_dir: cli.share_dir,
            project_dir: cli.project_dir,
        }
    }
}

pub struct App {
    preset_config: PresetConfig,
    project_config: ProjectConfig,
}

impl App {
    pub fn new(config: Config) -> anyhow::Result<Self> {
        cyak_core::utils::check_dir_existence(&config.share_dir)?;

        let preset_dir = config.share_dir.join("presets").join("default");
        cyak_core::utils::check_dir_existence(&preset_dir)?;

        let preset_config = cyak_core::load_preset_config(&preset_dir)?;

        let project_dir = config.project_dir;
        // If project dir already exist
        if project_dir.exists() {
            return Error::ProjectDirExists(project_dir.clone()).anyhow_fail();
        }

        let project_config = ProjectConfig::default();

        Ok(Self {
            preset_config,
            project_config,
        })
    }
}
