pub mod error;
pub mod lang;
pub mod preset_config;
pub mod project_config;
pub mod version;

use std::fs::{self, File};
use std::io::{Read, Write};
use std::path::{Path, PathBuf};

use handlebars::Handlebars;

use crate::project_config::TargetKind;
pub use error::Error;
use fs_extra::dir::{CopyOptions, DirOptions};
use preset_config::PresetConfig;
pub use project_config::ProjectConfig;

pub const LICENSE_FILE: &str = "LICENSE";

pub const CYAK_CONFIG_DIR: &str = ".cyak";
pub const CYAK_CONFIG_FILE: &str = ".cyak.yaml";

pub const ASIS_DIR: &str = "asis";

pub const PRESET_CONFIG_FILE: &str = "config.yaml";
pub const TEMPLATES_DIR: &str = "templates";
pub const CONFIG_TEMPLATE_FILE: &str = "config.hbs";
pub const EXECUTABLE_TEMPLATE_FILE: &str = "executable.hbs";
pub const INTERFACE_TEMPLATE_FILE: &str = "interface.hbs";
pub const LIBRARY_TEMPLATE_FILE: &str = "library.hbs";
pub const PROJECT_TEMPLATE_FILE: &str = "project.hbs";
pub const TEST_TEMPLATE_FILE: &str = "test.hbs";

pub const SOURCE_DIR: &str = "src";
pub const INTERFACE_DIR: &str = "include";
pub const CMAKE_FILE: &str = "CMakeLists.txt";

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

    // Create project directory
    fs::create_dir_all(&ctx.project_dir)?;

    // Git init
    if ctx.git {
        git_init(&ctx.project_dir)?;
    }

    let cyak_config_dir = ctx.project_dir.join(CYAK_CONFIG_DIR);
    let cyak_config_file_path = cyak_config_dir.join(CYAK_CONFIG_FILE);

    // Create cyak directory with config and actual preset
    fs::create_dir_all(&cyak_config_dir)?;
    let cyak_config_file = File::create(&cyak_config_file_path)?;
    serde_yaml::to_writer(cyak_config_file, &ctx.project_config)?;

    // Copy preset to cyak directory
    copy_preset_to_project(&ctx.preset_dir, &ctx.project_dir)?;

    // Copy `asis` directory to project directory
    copy_asis_to_project(&ctx.preset_dir, &ctx.project_dir)?;

    // Create license file if need it
    create_license(&ctx.project_dir, ctx.license)?;

    // Create all project
    create_project_from_config(&ctx.project_dir, &ctx.preset_dir, ctx.project_config)?;

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

pub fn git_init<P: AsRef<Path>>(dir: P) -> Result<(), Error> {
    git2::Repository::init(dir.as_ref())?;
    Ok(())
}

pub fn copy_preset_to_project<P: AsRef<Path>>(preset_dir: P, project_dir: P) -> Result<(), Error> {
    let preset_dir = preset_dir.as_ref();
    let project_dir = project_dir.as_ref();
    let cyak_config_dir = project_dir.join(CYAK_CONFIG_DIR);

    // Create project dir if not exist
    if !project_dir.exists() {
        fs::create_dir_all(&project_dir)?;
    }

    if !project_dir.is_dir() {
        return Error::NotDir(project_dir.to_path_buf()).fail();
    }

    if !preset_dir.is_dir() {
        return Error::NotDir(preset_dir.to_path_buf()).fail();
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

pub fn create_license<P: AsRef<Path>>(
    project_dir: P,
    license: Option<String>,
) -> Result<(), Error> {
    let project_dir = project_dir.as_ref();

    match license {
        None => { /*do nothing*/ }
        Some(s) => {
            let license_path = project_dir.join(LICENSE_FILE);
            let mut file = File::create(license_path)?;
            file.write_all(s.as_bytes())?;
        }
    }

    Ok(())
}

pub fn copy_asis_to_project<P: AsRef<Path>>(preset_dir: P, project_dir: P) -> Result<(), Error> {
    let preset_dir = preset_dir.as_ref();
    let project_dir = project_dir.as_ref();
    let asis_dir = preset_dir.join(ASIS_DIR);

    // Create project dir if not exist
    if !project_dir.exists() {
        fs::create_dir_all(&project_dir)?;
    }

    if !project_dir.is_dir() {
        return Error::NotDir(project_dir.to_path_buf()).fail();
    }

    if !preset_dir.is_dir() {
        return Error::NotDir(preset_dir.to_path_buf()).fail();
    }

    // If there is no `asis` dir then return
    if !asis_dir.exists() {
        return Ok(());
    }

    let asis_targets = {
        let opts = {
            let mut opts = DirOptions::new();
            opts.depth = 1;
            opts
        };
        let mut asis_content = fs_extra::dir::get_dir_content2(&asis_dir, &opts)?;
        asis_content.directories = asis_content
            .directories
            .into_iter()
            .filter(|s| !s.ends_with(ASIS_DIR))
            .collect();

        let mut targets: Vec<String> = Vec::new();
        targets.append(&mut asis_content.files);
        targets.append(&mut asis_content.directories);
        targets
    };

    // Copy all `asis` content
    {
        let mut opts = CopyOptions::new();
        opts.overwrite = true;
        fs_extra::copy_items(&asis_targets, &project_dir, &opts)?;
    }

    Ok(())
}

pub fn create_project_from_config<P: AsRef<Path>>(
    project_dir: P,
    preset_dir: P,
    project_config: ProjectConfig,
) -> Result<(), Error> {
    let project_dir = project_dir.as_ref();
    let preset_dir = preset_dir.as_ref();

    let templates_dir = preset_dir.join(TEMPLATES_DIR);
    let project_template = templates_dir.join(PROJECT_TEMPLATE_FILE);
    let config_template = templates_dir.join(CONFIG_TEMPLATE_FILE);
    let lib_template = templates_dir.join(LIBRARY_TEMPLATE_FILE);
    let exec_template = templates_dir.join(EXECUTABLE_TEMPLATE_FILE);
    let interface_template = templates_dir.join(INTERFACE_TEMPLATE_FILE);
    let test_template = templates_dir.join(TEST_TEMPLATE_FILE);

    let src_dir = project_dir.join(SOURCE_DIR);
    let interface_dir = project_dir.join(INTERFACE_DIR);

    // Validate again preset
    if !templates_dir.exists() {
        return Error::DirNotFound(templates_dir.clone()).fail();
    }

    if !project_template.exists() {
        return Error::FileNotFound(project_template.clone()).fail();
    }

    if !config_template.exists() {
        return Error::FileNotFound(config_template.clone()).fail();
    }

    if !lib_template.exists() {
        return Error::FileNotFound(lib_template.clone()).fail();
    }

    if !exec_template.exists() {
        return Error::FileNotFound(exec_template.clone()).fail();
    }

    if !interface_template.exists() {
        return Error::FileNotFound(interface_template.clone()).fail();
    }

    if !test_template.exists() {
        return Error::FileNotFound(test_template.clone()).fail();
    }

    if !project_dir.exists() {
        fs::create_dir_all(&project_dir)?;
    }

    let mut reg = Handlebars::new();

    // Create main CMakeLists.txt
    {
        let mut project_file = File::open(&project_template)?;
        let mut project_file_dest = File::create(&project_dir.join(CMAKE_FILE))?;
        reg.render_template_source_to_write(&mut project_file, &project_config, project_file_dest)?;
    }

    // Create targets CMakeLists.txt
    {
        for target in &project_config.targets {
            // Create source dir or interface if not exists
            let base_dir = match target.kind {
                TargetKind::Executable => {
                    let dir = src_dir.join(&target.name);
                    if !dir.exists() {
                        fs::create_dir_all(&dir)?;
                    }
                    dir
                }
                TargetKind::Library => {
                    let dir = src_dir.join(&target.name);
                    if !dir.exists() {
                        fs::create_dir_all(&dir)?;
                    }
                    dir
                }
                TargetKind::Interface => {
                    let dir = interface_dir.join(&target.name);
                    if !dir.exists() {
                        fs::create_dir_all(&dir)?;
                    }
                    dir
                }
            };

            let mut file = match target.kind {
                TargetKind::Executable => File::open(&exec_template)?,
                TargetKind::Library => File::open(&lib_template)?,
                TargetKind::Interface => File::open(&interface_template)?,
            };
            let mut file_dest = match target.kind {
                TargetKind::Executable => File::create(&base_dir.join(CMAKE_FILE))?,
                TargetKind::Library => File::create(&base_dir.join(CMAKE_FILE))?,
                TargetKind::Interface => File::create(&base_dir.join(CMAKE_FILE))?,
            };
            reg.render_template_source_to_write(&mut file, target, file_dest)?;
        }
    }

    Ok(())
}
