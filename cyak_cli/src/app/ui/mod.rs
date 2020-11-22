pub mod error;

pub use error::*;

use cyak_core::{Context, PresetConfig, ProjectConfig};
use std::path::{Path, PathBuf};

pub struct Ui {
    pub share_dir: PathBuf,
    pub project_dir: PathBuf,
    pub preset_dir: PathBuf,
    pub git: bool,
    pub license: Option<String>,
    pub project_config: ProjectConfig,
    pub preset_config: PresetConfig,
}

impl Ui {
    pub fn new<P1, P2>(share_dir: P1, project_dir: P2) -> anyhow::Result<Ui>
    where
        P1: AsRef<Path>,
        P2: AsRef<Path>,
    {
        let share_dir = share_dir.as_ref();
        let project_dir = project_dir.as_ref();

        cyak_core::utils::check_dir_existence(&share_dir)?;

        let preset_dir = share_dir.join("presets").join("default");
        cyak_core::utils::check_dir_existence(&preset_dir)?;

        let preset_config = cyak_core::load_preset_config(&preset_dir)?;

        // If project dir already exist
        if project_dir.exists() {
            return Error::Fatal(format!(
                "Project directory already exists: {:?}",
                project_dir
            ))
            .anyhow_fail();
        }

        let project_config = {
            let mut p = ProjectConfig::default();
            p.init_variables(&preset_config);
            p
        };

        Ok(Ui {
            share_dir: share_dir.to_path_buf(),
            project_dir: project_dir.to_path_buf(),
            preset_dir,
            git: true,
            license: None,
            project_config,
            preset_config,
        })
    }
}
