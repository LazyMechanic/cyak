use cyak_core::lang::Language;
use cyak_core::project_config::{Target, TargetKind, TargetProperty};
use cyak_core::version::Version;
use cyak_core::ProjectConfig;
use uuid::Uuid;

mod common;

#[test]
fn check_already_generated_project() -> anyhow::Result<()> {
    let project_dir = common::create_mock_project()?;
    assert!(cyak_core::is_project_already_generated(project_dir));
    Ok(())
}

#[test]
fn create_project_from_config() -> anyhow::Result<()> {
    let config = ProjectConfig {
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
    };

    let project_dir = common::finalize_path(&Uuid::new_v4().to_string());
    let preset_dir = common::create_mock_test_preset()?;

    std::fs::create_dir_all(&project_dir)?;

    cyak_core::create_project_from_config(&project_dir, &preset_dir, config)?;

    Ok(())
}
