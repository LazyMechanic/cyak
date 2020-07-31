pub mod consts;
pub mod context;
pub mod error;
pub mod lang;
pub mod preset;
pub mod project;
pub mod utils;
pub mod version;

use std::fs::{self, File};
use std::io::{Read, Write};
use std::path::Path;

use fs_extra::dir::{CopyOptions, DirOptions};
use handlebars::Handlebars;

use context::Context;
use preset::PresetConfig;

pub use consts::*;
pub use error::Error;
pub use project::{ProjectConfig, Target, TargetKind, TargetProperty};
pub use version::Version;

pub fn create_project(ctx: Context) -> Result<(), Error> {
    if is_project_already_created(&ctx.project_dir) {
        return Error::ProjectAlreadyGenerated(ctx.project_dir).fail();
    }

    validate_preset(&ctx.preset_dir)?;

    // Create project directory
    fs::create_dir_all(&ctx.project_dir)?;

    // Git init
    if ctx.git {
        git_init(&ctx.project_dir)?;
    }

    // Copy preset to cyak directory
    copy_preset_to_project(&ctx.preset_dir, &ctx.project_dir)?;

    // Copy `asis` directory to project directory
    copy_asis_to_project(&ctx.preset_dir, &ctx.project_dir)?;

    // Create license file if need it
    create_license(&ctx.project_dir, ctx.license)?;

    // Create project from config
    create_project_from_config(&ctx.project_dir, &ctx.preset_dir, &ctx.project_config)?;

    // Save config to project
    save_project_config(&ctx.project_dir, &ctx.project_config)?;

    Ok(())
}

pub fn is_project_already_created<P: AsRef<Path>>(project_dir: P) -> bool {
    let project_dir = project_dir.as_ref();

    let config_file = utils::format_project_config(&project_dir);
    config_file.exists()
}

pub fn validate_preset<P: AsRef<Path>>(preset_dir: P) -> Result<(), Error> {
    let preset_dir = preset_dir.as_ref();
    let mut error_files = Vec::new();

    // Check config.yaml
    let preset_config = utils::format_preset_config(&preset_dir);
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

    utils::check_dir_existence(&preset_dir)?;

    // Create all path if not exists
    utils::create_nonexistent_dir_all(&cyak_config_dir)?;

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
    let asis_dir = utils::format_asis_dir(&preset_dir);

    utils::check_dir_existence(&preset_dir)?;

    // Create all path if not exists
    utils::create_nonexistent_dir_all(&project_dir)?;

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
    project_config: &ProjectConfig,
) -> Result<(), Error> {
    let project_dir = project_dir.as_ref();
    let preset_dir = preset_dir.as_ref();

    let project_template = utils::format_project_template(&preset_dir);
    let config_template = utils::format_config_template(&preset_dir);
    let lib_template = utils::format_library_template(&preset_dir);
    let exec_template = utils::format_executable_template(&preset_dir);
    let interface_template = utils::format_interface_template(&preset_dir);
    let test_template = utils::format_test_template(&preset_dir);

    utils::check_dir_existence(&preset_dir)?;
    utils::check_file_existence(&project_template)?;
    utils::check_file_existence(&config_template)?;
    utils::check_file_existence(&lib_template)?;
    utils::check_file_existence(&exec_template)?;
    utils::check_file_existence(&interface_template)?;
    utils::check_file_existence(&test_template)?;

    let src_dir = project_dir.join(SOURCE_DIR);
    let tests_dir = project_dir.join(TESTS_DIR);
    let interface_dir = project_dir.join(INTERFACE_DIR);

    // Create all path if not exists
    utils::create_nonexistent_dir_all(&project_dir)?;

    let reg = Handlebars::new();

    // Create main CMakeLists.txt
    {
        let mut project_file = File::open(&project_template)?;
        let project_file_dest = File::create(&project_dir.join(CMAKE_FILE))?;
        reg.render_template_source_to_write(&mut project_file, &project_config, project_file_dest)?;
    }

    // Create targets CMakeLists.txt
    {
        for target in &project_config.targets {
            // Create source dir or interface if not exists
            let base_dir = match target.kind {
                TargetKind::Executable => {
                    let dir = src_dir.join(&target.name);
                    utils::create_nonexistent_dir_all(&dir)?;

                    // Add empty file with hello world
                    let mut file = File::create(dir.join(EXEC_SRC_FILE))?;
                    file.write_all(EXEC_SRC.as_bytes())?;
                    dir
                }
                TargetKind::Library => {
                    let dir = src_dir.join(&target.name);
                    utils::create_nonexistent_dir_all(&dir)?;

                    // Add empty file with hello world
                    let mut file = File::create(dir.join(&target.name))?;
                    file.write_all(LIB_SRC.as_bytes())?;
                    dir
                }
                TargetKind::Interface => {
                    let dir = interface_dir.join(&target.name);
                    utils::create_nonexistent_dir_all(&dir)?;

                    // Add empty file with hello world
                    let mut file = File::create(dir.join(&target.name))?;
                    file.write_all(LIB_SRC.as_bytes())?;
                    dir
                }
                TargetKind::Test => {
                    let dir = tests_dir.join(&target.name);
                    utils::create_nonexistent_dir_all(&dir)?;

                    // Add empty file with hello world
                    let mut file = File::create(dir.join(EXEC_SRC_FILE))?;
                    file.write_all(EXEC_SRC.as_bytes())?;
                    dir
                }
            };

            // Create target source dir and CMakeLists.txt file
            let mut file = match target.kind {
                TargetKind::Executable => File::open(&exec_template)?,
                TargetKind::Library => File::open(&lib_template)?,
                TargetKind::Interface => File::open(&interface_template)?,
                TargetKind::Test => File::open(&test_template)?,
            };
            let file_dest = match target.kind {
                TargetKind::Executable => File::create(&base_dir.join(CMAKE_FILE))?,
                TargetKind::Library => File::create(&base_dir.join(CMAKE_FILE))?,
                TargetKind::Interface => File::create(&base_dir.join(CMAKE_FILE))?,
                TargetKind::Test => File::create(&base_dir.join(CMAKE_FILE))?,
            };
            reg.render_template_source_to_write(&mut file, target, file_dest)?;

            // Create lib config file to cmake modules
            match target.kind {
                TargetKind::Library | TargetKind::Interface => {
                    let cmake_modules_dir = utils::format_cmake_modules_dir(&project_dir);
                    let lib_config_file =
                        utils::format_lib_config_file(&cmake_modules_dir, target.name.as_ref());

                    utils::create_nonexistent_dir_all(&cmake_modules_dir)?;

                    let mut file = File::open(&config_template)?;
                    let file_dest = File::create(&lib_config_file)?;

                    reg.render_template_source_to_write(&mut file, target, file_dest)?;
                }
                _ => { /*do nothing*/ }
            }
        }
    }

    Ok(())
}

pub fn save_project_config<P: AsRef<Path>>(
    project_dir: P,
    project_config: &ProjectConfig,
) -> Result<(), Error> {
    let project_dir = project_dir.as_ref();
    let cyak_file = utils::format_project_config(&project_dir);

    utils::check_dir_existence(&project_dir)?;

    let f = File::create(cyak_file)?;
    serde_yaml::to_writer(f, project_config)?;

    Ok(())
}
