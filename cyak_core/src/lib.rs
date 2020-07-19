pub mod error;
pub mod lang;
pub mod preset_config;
pub mod project_config;
pub mod version;

pub use error::Error;

use project_config::ProjectConfig;
use std::path::{Path, PathBuf};

pub const CYAK_CONFIG_DIR: &str = ".cyak";
pub const CYAK_CONFIG_FILE: &str = ".cyak.yaml";

pub const PRESET_CONFIG_FILE: &str = "config.yaml";
pub const TEMPLATES_DIR: &str = "templates";
pub const CONFIG_TEMPLATE_FILE: &str = "config.hbs";
pub const EXECUTABLE_TEMPLATE_FILE: &str = "executable.hbs";
pub const INTERFACE_TEMPLATE_FILE: &str = "interface.hbs";
pub const LIBRARY_TEMPLATE_FILE: &str = "library.hbs";
pub const PROJECT_TEMPLATE_FILE: &str = "project.hbs";
pub const TEST_TEMPLATE_FILE: &str = "test.hbs";

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

    let config_file = dir.join(CYAK_CONFIG_DIR).join(CYAK_CONFIG_FILE);
    config_file.exists()
}

pub fn validate_preset<P: AsRef<Path>>(dir: P) -> Result<(), Error> {
    let preset_dir = dir.as_ref();
    let mut missing_files = Vec::new();

    // Check config.yaml
    let preset_config = preset_dir.join(Path::new(PRESET_CONFIG_FILE));
    if !preset_config.exists() {
        missing_files.push(preset_config);
    }

    // Check templates dir
    let templates_dir = preset_dir.join(TEMPLATES_DIR);
    if !templates_dir.exists() {
        missing_files.push(templates_dir.clone());
    }

    // Check project template
    let project_template = templates_dir.join(PROJECT_TEMPLATE_FILE);
    if !project_template.exists() {
        missing_files.push(project_template);
    }

    // Check library config template
    let config_template = templates_dir.join(CONFIG_TEMPLATE_FILE);
    if !config_template.exists() {
        missing_files.push(config_template);
    }

    // Check library template
    let lib_template = templates_dir.join(LIBRARY_TEMPLATE_FILE);
    if !lib_template.exists() {
        missing_files.push(lib_template);
    }

    // Check executable template
    let exec_template = templates_dir.join(EXECUTABLE_TEMPLATE_FILE);
    if !exec_template.exists() {
        missing_files.push(exec_template);
    }

    // Check interface template
    let interface_template = templates_dir.join(INTERFACE_TEMPLATE_FILE);
    if !interface_template.exists() {
        missing_files.push(interface_template);
    }

    // Check test template
    let test_template = templates_dir.join(TEST_TEMPLATE_FILE);
    if !test_template.exists() {
        missing_files.push(test_template);
    }

    if missing_files.len() > 0 {
        return Error::InvalidPresetStructure(missing_files).fail();
    }

    Ok(())
}
