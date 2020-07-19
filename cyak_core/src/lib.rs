pub mod error;
pub mod lang;
pub mod preset_config;
pub mod project_config;
pub mod version;

use std::fs::{self, File};
use std::io::{self, Read};
use std::path::{Path, PathBuf};
use std::process;

pub use error::Error;
use preset_config::PresetConfig;
pub use project_config::ProjectConfig;

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

    validate_preset(&ctx.preset_dir)?;

    // Make project directory
    fs::create_dir_all(&ctx.project_dir)?;

    let cyak_config_dir = ctx.project_dir.join(CYAK_CONFIG_DIR);
    let cyak_config_file_path = cyak_config_dir.join(CYAK_CONFIG_FILE);

    // Make cyak directory with config and actual preset
    fs::create_dir_all(&cyak_config_dir)?;
    let cyak_config_file = File::create(&cyak_config_file_path)?;
    serde_yaml::to_writer(cyak_config_file, &ctx.project_config)?;

    // Copy preset to cyak directory
    copy_preset_to_project(&ctx.preset_dir, &ctx.project_dir)?;

    Ok(())
}

pub fn copy_preset_to_project<P: AsRef<Path>>(preset_dir: P, project_dir: P) -> Result<(), Error> {
    let preset_dir = preset_dir.as_ref();
    let project_dir = project_dir.as_ref();
    let cyak_config_dir = project_dir.join(CYAK_CONFIG_DIR);

    if !preset_dir.is_dir() {
        return Error::NotDir(preset_dir.to_path_buf()).fail();
    }

    if !project_dir.is_dir() {
        return Error::NotDir(project_dir.to_path_buf()).fail();
    }

    // Make cyak directory if not exist
    if !cyak_config_dir.exists() {
        fs::create_dir(&cyak_config_dir)?;
    }

    // Copy preset directory recursively
    fs_extra::dir::copy(
        preset_dir,
        &cyak_config_dir,
        &fs_extra::dir::CopyOptions::new(),
    )?;

    Ok(())
}

pub fn is_project_already_generated<P: AsRef<Path>>(dir: P) -> bool {
    let dir = dir.as_ref();

    let config_file = dir.join(CYAK_CONFIG_DIR).join(CYAK_CONFIG_FILE);
    config_file.exists()
}

pub fn validate_preset<P: AsRef<Path>>(dir: P) -> Result<(), Error> {
    let preset_dir = dir.as_ref();
    let mut error_files = Vec::new();

    // Check config.yaml
    let preset_config = preset_dir.join(Path::new(PRESET_CONFIG_FILE));
    if !preset_config.exists() {
        error_files.push(("missing file".to_string(), preset_config));
    } else {
        let mut f = File::open(&preset_config)?;
        let mut s = String::new();
        f.read_to_string(&mut s)?;
        match serde_yaml::from_str::<PresetConfig>(&s) {
            Ok(_) => { /*do nothing*/ }
            Err(e) => error_files.push((e.to_string(), preset_config)),
        }
    }

    // Check templates dir
    let templates_dir = preset_dir.join(TEMPLATES_DIR);
    if !templates_dir.exists() {
        error_files.push(("missing file".to_string(), templates_dir.clone()));
    }

    // Check project template
    let project_template = templates_dir.join(PROJECT_TEMPLATE_FILE);
    if !project_template.exists() {
        error_files.push(("missing file".to_string(), project_template));
    }

    // Check library config template
    let config_template = templates_dir.join(CONFIG_TEMPLATE_FILE);
    if !config_template.exists() {
        error_files.push(("missing file".to_string(), config_template));
    }

    // Check library template
    let lib_template = templates_dir.join(LIBRARY_TEMPLATE_FILE);
    if !lib_template.exists() {
        error_files.push(("missing file".to_string(), lib_template));
    }

    // Check executable template
    let exec_template = templates_dir.join(EXECUTABLE_TEMPLATE_FILE);
    if !exec_template.exists() {
        error_files.push(("missing file".to_string(), exec_template));
    }

    // Check interface template
    let interface_template = templates_dir.join(INTERFACE_TEMPLATE_FILE);
    if !interface_template.exists() {
        error_files.push(("missing file".to_string(), interface_template));
    }

    // Check test template
    let test_template = templates_dir.join(TEST_TEMPLATE_FILE);
    if !test_template.exists() {
        error_files.push(("missing file".to_string(), test_template));
    }

    if error_files.len() > 0 {
        return Error::InvalidPresetStructure(error_files).fail();
    }

    Ok(())
}
