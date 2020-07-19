use cyak_core::Error;
use uuid::Uuid;

mod common;

#[test]
fn validate_valid_preset() -> anyhow::Result<()> {
    let preset_dir = common::create_mock_preset()?;
    assert!(
        !cyak_core::validate_preset(&preset_dir).is_err(),
        "preset should be valid"
    );
    Ok(())
}

#[test]
fn validate_invalid_preset() -> anyhow::Result<()> {
    let preset_dir = common::create_mock_invalid_preset()?;
    assert!(
        cyak_core::validate_preset(&preset_dir).is_err(),
        "preset should be invalid"
    );
    Ok(())
}

#[test]
fn copy_preset_to_project() -> anyhow::Result<()> {
    let preset_dir = common::create_mock_preset()?;
    let project_dir = common::finalize_path(&Uuid::new_v4().to_string());
    std::fs::create_dir_all(&project_dir)?;

    assert!(
        !cyak_core::copy_preset_to_project(&preset_dir, &project_dir).is_err(),
        "copy should be successful"
    );

    let preset_name = preset_dir
        .file_name()
        .ok_or(Error::InvalidFilename(preset_dir.clone()))?;
    let preset_new_dir = project_dir
        .join(cyak_core::CYAK_CONFIG_DIR)
        .join(preset_name);

    assert!(!dir_diff::is_different(preset_dir, preset_new_dir).unwrap());

    Ok(())
}
