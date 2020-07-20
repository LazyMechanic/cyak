use std::fs;
use std::fs::File;
use std::path::{Path, PathBuf};

use std::io::Write;
use uuid::Uuid;

pub const STUFF_DIR: &str = "tests/stuff";

#[allow(dead_code)]
pub fn finalize_path<P: AsRef<Path>>(path: P) -> PathBuf {
    let dir = PathBuf::from(STUFF_DIR);
    dir.join(path)
}

#[allow(dead_code)]
pub fn create_mock_project() -> anyhow::Result<PathBuf> {
    let project_dir = finalize_path(&Uuid::new_v4().to_string());
    let config_dir = project_dir.join(cyak_core::CYAK_CONFIG_DIR);
    let file = config_dir.join(cyak_core::CYAK_CONFIG_FILE);

    fs::create_dir_all(&config_dir)?;
    File::create(file)?;

    Ok(project_dir)
}

#[allow(dead_code)]
pub fn create_mock_preset() -> anyhow::Result<PathBuf> {
    let preset_dir = finalize_path(&Uuid::new_v4().to_string());
    let templates_dir = preset_dir.join(cyak_core::TEMPLATES_DIR);
    let asis_dir = preset_dir.join(cyak_core::ASIS_DIR);

    let config_file = preset_dir.join(cyak_core::PRESET_CONFIG_FILE);

    let config_template = templates_dir.join(cyak_core::CONFIG_TEMPLATE_FILE);
    let project_template = templates_dir.join(cyak_core::PROJECT_TEMPLATE_FILE);
    let exec_template = templates_dir.join(cyak_core::EXECUTABLE_TEMPLATE_FILE);
    let lib_template = templates_dir.join(cyak_core::LIBRARY_TEMPLATE_FILE);
    let interface_template = templates_dir.join(cyak_core::INTERFACE_TEMPLATE_FILE);
    let test_template = templates_dir.join(cyak_core::TEST_TEMPLATE_FILE);

    // Create preset directory
    fs::create_dir_all(&preset_dir)?;

    // Create `asis`
    {
        fs::create_dir(&asis_dir)?;

        File::create(&asis_dir.join(&Uuid::new_v4().to_string()))?;
        File::create(&asis_dir.join(&Uuid::new_v4().to_string()))?;
        fs::create_dir(&asis_dir.join(&Uuid::new_v4().to_string()))?;
        fs::create_dir(&asis_dir.join(&Uuid::new_v4().to_string()))?;
        {
            let dir = asis_dir.join(&Uuid::new_v4().to_string());
            let nested_dir1 = dir.join(&Uuid::new_v4().to_string());
            let nested_dir2 = dir.join(&Uuid::new_v4().to_string());

            fs::create_dir(&dir)?;
            fs::create_dir(&nested_dir1)?;
            fs::create_dir(&nested_dir2)?;

            File::create(&dir.join(&Uuid::new_v4().to_string()))?;
            File::create(&nested_dir1.join(&Uuid::new_v4().to_string()))?;
            File::create(&nested_dir1.join(&Uuid::new_v4().to_string()))?;
            File::create(&nested_dir2.join(&Uuid::new_v4().to_string()))?;
            File::create(&nested_dir2.join(&Uuid::new_v4().to_string()))?;
        }
    }

    // Create templates
    {
        fs::create_dir_all(&templates_dir)?;

        let mut c = File::create(&config_file)?;
        {
            let config_text = r#"---
name: "Default"
version: "1.0.0"
author: "LazyMechanic"
description: "Default preset for cross platform C++ (by default) project"
default_values:
  language: "CXX"
  version:
    major: 0
    minor: 1
    patch: 0
  git: true
  target_properties:
    custom:
      - display: "Language standard"
        description: "Set language standard"
        key: "CXX_STANDARD"
        value_pattern: "^[0-9]+$"
        default: "17"
      - display: "Language extensions"
        description: "Enable or disable extensions. For example GCC extensions"
        key: "CXX_EXTENSIONS"
        value_pattern: "on|off"
        default: "off"
    common:
      - key: "CXX_STANDARD_REQUIRED"
        value: "ON"
"#;
            c.write_all(config_text.as_bytes())?;
        }

        File::create(&config_template)?;
        File::create(&project_template)?;
        File::create(&exec_template)?;
        File::create(&lib_template)?;
        File::create(&interface_template)?;
        File::create(&test_template)?;
    }

    Ok(preset_dir)
}

#[allow(dead_code)]
pub fn create_mock_invalid_preset() -> anyhow::Result<PathBuf> {
    let preset_dir = finalize_path(&Uuid::new_v4().to_string());
    let templates_dir = preset_dir.join(cyak_core::TEMPLATES_DIR);
    let config_file = preset_dir.join(cyak_core::PRESET_CONFIG_FILE);

    //let config_template = templates_dir.join(cyak_core::CONFIG_TEMPLATE_FILE);
    let project_template = templates_dir.join(cyak_core::PROJECT_TEMPLATE_FILE);
    //let exec_template = templates_dir.join(cyak_core::EXECUTABLE_TEMPLATE_FILE);
    let lib_template = templates_dir.join(cyak_core::LIBRARY_TEMPLATE_FILE);
    let interface_template = templates_dir.join(cyak_core::INTERFACE_TEMPLATE_FILE);
    // let test_template = templates_dir.join(cyak_core::TEST_TEMPLATE_FILE);

    // Create structure
    fs::create_dir_all(&preset_dir)?;
    fs::create_dir_all(&templates_dir)?;

    let mut c = File::create(&config_file)?;
    {
        let config_text = r#"---
name: "Default"
version: "1.0.0"
author: "LazyMechanic"
description: "Default preset for cross platform C++ (by default) project"
"#;
        c.write_all(config_text.as_bytes())?;
    }

    //File::create(&config_template)?;
    File::create(&project_template)?;
    //File::create(&exec_template)?;
    File::create(&lib_template)?;
    File::create(&interface_template)?;
    //File::create(&test_template)?;

    Ok(preset_dir)
}
