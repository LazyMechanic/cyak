pub mod error;
pub mod lang;
pub mod preset_config;
pub mod project_config;
pub mod version;

pub use error::Error;

use project_config::ProjectConfig;
use std::path::{Path, PathBuf};

#[derive(Debug)]
pub struct Context {
    pub project_dir: PathBuf,
    pub preset_dir: PathBuf,
    pub git: bool,
    pub license: Option<String>,
    pub project_config: ProjectConfig,
}

pub fn generate_project(ctx: Context) -> Result<(), Error> {
    if is_project_already_generated(&ctx.project_dir) {
        return Error::ProjectAlreadyGenerated(ctx.project_dir).fail();
    }

    Ok(())
}

pub fn is_project_already_generated<P: AsRef<Path>>(dir: P) -> bool {
    let dir = dir.as_ref();

    const PROJECT_CONFIG_DIR: &str = ".cyak";
    const PROJECT_CONFIG_FILE: &str = ".cyak.yaml";

    let config_file = dir.join(PROJECT_CONFIG_DIR).join(PROJECT_CONFIG_FILE);
    config_file.exists()
}
