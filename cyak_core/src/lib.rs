pub mod error;
pub mod lang;
pub mod preset_config;
pub mod project_config;
pub mod version;

pub use error::Error;

use project_config::ProjectConfig;
use std::path::{Path, PathBuf};

pub const PROJECT_CONFIG_DIR: &str = ".cyak";
pub const PROJECT_CONFIG_FILE: &str = ".cyak.yaml";

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

    let config_file = dir.join(PROJECT_CONFIG_DIR).join(PROJECT_CONFIG_FILE);
    config_file.exists()
}
