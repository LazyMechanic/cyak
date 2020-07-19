use std::fs;
use std::fs::File;
use std::path::{Path, PathBuf};

use uuid::Uuid;

pub const STUFF_DIR: &str = "tests/stuff";

pub fn finalize_path<P: AsRef<Path>>(path: P) -> PathBuf {
    let dir = PathBuf::from(STUFF_DIR);
    dir.join(path)
}

pub fn create_mock_project() -> anyhow::Result<PathBuf> {
    let project_dir = finalize_path(&Uuid::new_v4().to_string());
    let config_dir = project_dir.join(cyak_core::CYAK_CONFIG_DIR);
    let file = config_dir.join(cyak_core::CYAK_CONFIG_FILE);

    fs::create_dir_all(&config_dir)?;
    File::create(file)?;

    Ok(project_dir)
}

pub fn create_mock_preset() -> anyhow::Result<PathBuf> {
    let preset_dir = finalize_path(&Uuid::new_v4().to_string());
    let templates_dir = preset_dir.join(cyak_core::TEMPLATES_DIR);
    let config_file = preset_dir.join(cyak_core::PRESET_CONFIG_FILE);

    let config_template = templates_dir.join(cyak_core::CONFIG_TEMPLATE_FILE);
    let project_template = templates_dir.join(cyak_core::PROJECT_TEMPLATE_FILE);
    let exec_template = templates_dir.join(cyak_core::EXECUTABLE_TEMPLATE_FILE);
    let lib_template = templates_dir.join(cyak_core::LIBRARY_TEMPLATE_FILE);
    let interface_template = templates_dir.join(cyak_core::INTERFACE_TEMPLATE_FILE);
    let test_template = templates_dir.join(cyak_core::TEST_TEMPLATE_FILE);

    // Create structure
    fs::create_dir_all(&preset_dir)?;
    fs::create_dir_all(&templates_dir)?;

    File::create(&config_file)?;
    File::create(&config_template)?;
    File::create(&project_template)?;
    File::create(&exec_template)?;
    File::create(&lib_template)?;
    File::create(&interface_template)?;
    File::create(&test_template)?;

    Ok(preset_dir)
}

pub fn create_mock_invalid_preset() -> anyhow::Result<PathBuf> {
    let preset_dir = finalize_path(&Uuid::new_v4().to_string());
    let templates_dir = preset_dir.join(cyak_core::TEMPLATES_DIR);
    //let config_file = preset_dir.join(cyak_core::PRESET_CONFIG_FILE);

    let config_template = templates_dir.join(cyak_core::CONFIG_TEMPLATE_FILE);
    let project_template = templates_dir.join(cyak_core::PROJECT_TEMPLATE_FILE);
    //let exec_template = templates_dir.join(cyak_core::EXECUTABLE_TEMPLATE_FILE);
    let lib_template = templates_dir.join(cyak_core::LIBRARY_TEMPLATE_FILE);
    let interface_template = templates_dir.join(cyak_core::INTERFACE_TEMPLATE_FILE);
    // let test_template = templates_dir.join(cyak_core::TEST_TEMPLATE_FILE);

    // Create structure
    fs::create_dir_all(&preset_dir)?;
    fs::create_dir_all(&templates_dir)?;

    //File::create(&config_file)?;
    File::create(&config_template)?;
    File::create(&project_template)?;
    //File::create(&exec_template)?;
    File::create(&lib_template)?;
    File::create(&interface_template)?;
    //File::create(&test_template)?;

    Ok(preset_dir)
}
