use std::fs;
use std::fs::File;
use std::path::{Path, PathBuf};

use cyak_core::consts::*;
use cyak_core::lang::Language;
use cyak_core::{utils, ProjectConfig, Target, TargetKind, TargetProperty, Version};
use handlebars::Handlebars;
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

const PROJECT_TEMPLATE: &str = r#"
Name          = {{name}}
Namespace     = {{namespace}}
Version.major = {{version.major}}
Version.minor = {{version.minor}}
Version.patch = {{version.patch}}
Language      = {{language}}
Targets       = [
    {{#each targets}}{
        Kind          = {{kind}}
        Name          = {{name}}
        Version.major = {{version.major}}
        Version.minor = {{version.minor}}
        Version.patch = {{version.patch}}
        Properties    = [
            {{#each properties}}{
                Key   = {{key}}
                Value = {{value}}
            },
            {{/each}}
        ]
    },
    {{/each}}
"#;

const TARGET_TEMPLATE: &str = r#"
Kind          = {{kind}}
Name          = {{name}}
Version.major = {{version.major}}
Version.minor = {{version.minor}}
Version.patch = {{version.patch}}
Properties    = [
    {{#each properties}}{
        Key   = {{key}}
        Value = {{value}}
    },
    {{/each}}
]
"#;

#[allow(dead_code)]
pub fn create_mock_test_preset() -> anyhow::Result<PathBuf> {
    let preset_dir = finalize_path(&Uuid::new_v4().to_string());
    let templates_dir = preset_dir.join(cyak_core::TEMPLATES_DIR);
    // TODO: let asis_dir = preset_dir.join(cyak_core::ASIS_DIR);

    let config_file = preset_dir.join(cyak_core::PRESET_CONFIG_FILE);

    let config_template = templates_dir.join(cyak_core::CONFIG_TEMPLATE_FILE);
    let project_template = templates_dir.join(cyak_core::PROJECT_TEMPLATE_FILE);
    let exec_template = templates_dir.join(cyak_core::EXECUTABLE_TEMPLATE_FILE);
    let lib_template = templates_dir.join(cyak_core::LIBRARY_TEMPLATE_FILE);
    let interface_template = templates_dir.join(cyak_core::INTERFACE_TEMPLATE_FILE);
    let test_template = templates_dir.join(cyak_core::TEST_TEMPLATE_FILE);

    // Create preset directory
    fs::create_dir_all(&preset_dir)?;

    // Create templates
    {
        fs::create_dir_all(&templates_dir)?;

        {
            let mut c = File::create(&config_file)?;
            let config_text = r#"---
name: "NAME"
version: "1.0.0"
author: "AUTHOR"
description: "DESCRIPTION"
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

        {
            let mut f = File::create(&project_template)?;
            f.write_all(PROJECT_TEMPLATE.as_bytes())?;
        }
        {
            let mut f = File::create(&config_template)?;
            f.write_all(TARGET_TEMPLATE.as_bytes())?;
        }
        {
            let mut f = File::create(&exec_template)?;
            f.write_all(TARGET_TEMPLATE.as_bytes())?;
        }
        {
            let mut f = File::create(&lib_template)?;
            f.write_all(TARGET_TEMPLATE.as_bytes())?;
        }
        {
            let mut f = File::create(&interface_template)?;
            f.write_all(TARGET_TEMPLATE.as_bytes())?;
        }
        {
            let mut f = File::create(&test_template)?;
            f.write_all(TARGET_TEMPLATE.as_bytes())?;
        }
    }

    Ok(preset_dir)
}

#[allow(dead_code)]
pub fn new_mock_project_config() -> ProjectConfig {
    ProjectConfig {
        name: "project_name".to_string(),
        namespace: "project_namespace".to_string(),
        version: Version {
            major: 7,
            minor: 8,
            patch: 9,
        },
        language: Language::Cpp,
        targets: vec![
            Target {
                kind: TargetKind::Executable,
                name: "exec_name".to_string(),
                version: Version {
                    major: 6,
                    minor: 7,
                    patch: 6,
                },
                properties: vec![],
            },
            Target {
                kind: TargetKind::Library,
                name: "lib_name".to_string(),
                version: Version {
                    major: 1,
                    minor: 6,
                    patch: 5,
                },
                properties: vec![
                    TargetProperty {
                        key: "some_property1".to_string(),
                        value: "value1".to_string(),
                    },
                    TargetProperty {
                        key: "some_property2".to_string(),
                        value: "2".to_string(),
                    },
                ],
            },
            Target {
                kind: TargetKind::Interface,
                name: "interface_name".to_string(),
                version: Version {
                    major: 66,
                    minor: 7,
                    patch: 123,
                },
                properties: vec![
                    TargetProperty {
                        key: "some_property1".to_string(),
                        value: "value1".to_string(),
                    },
                    TargetProperty {
                        key: "some_property2".to_string(),
                        value: "2".to_string(),
                    },
                ],
            },
        ],
    }
}

#[allow(dead_code)]
pub fn create_mock_project_from_config<P: AsRef<Path>>(
    preset_dir: P,
    project_config: &ProjectConfig,
) -> anyhow::Result<PathBuf> {
    let project_dir = finalize_path(&Uuid::new_v4().to_string());
    let preset_dir = preset_dir.as_ref();

    let project_template = utils::format_project_template(&preset_dir);
    let config_template = utils::format_config_template(&preset_dir);
    let lib_template = utils::format_library_template(&preset_dir);
    let exec_template = utils::format_executable_template(&preset_dir);
    let interface_template = utils::format_interface_template(&preset_dir);
    let test_template = utils::format_test_template(&preset_dir);

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
                    let lib_config_file = utils::format_lib_config_file(
                        &cmake_modules_dir,
                        &PathBuf::from(&target.name),
                    );

                    utils::create_nonexistent_dir_all(&cmake_modules_dir)?;

                    let mut file = File::open(&config_template)?;
                    let file_dest = File::create(&lib_config_file)?;

                    reg.render_template_source_to_write(&mut file, target, file_dest)?;
                }
                _ => { /*do nothing*/ }
            }
        }
    }

    Ok(project_dir)
}
