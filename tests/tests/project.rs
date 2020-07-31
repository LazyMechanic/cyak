use uuid::Uuid;

mod common;

#[test]
fn check_already_created_project() -> anyhow::Result<()> {
    let project_dir = common::create_mock_project()?;
    assert!(cyak_core::is_project_already_created(project_dir));
    Ok(())
}

#[test]
fn create_project_from_config() -> anyhow::Result<()> {
    let config = common::new_mock_project_config();
    let preset_dir = common::create_mock_test_preset()?;

    let project_expected = common::create_mock_project_from_config(&preset_dir, &config)?;
    let project_actual = common::finalize_path(&Uuid::new_v4().to_string());

    cyak_core::create_project_from_config(&project_actual, &preset_dir, &config)?;

    assert!(!dir_diff::is_different(&project_actual, &project_expected).unwrap());

    Ok(())
}
