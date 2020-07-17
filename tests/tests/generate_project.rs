mod common;

#[test]
fn check_already_generated_project() -> anyhow::Result<()> {
    let project_dir_path = common::create_mock_project()?;
    assert!(cyak_core::is_project_already_generated(project_dir_path));
    Ok(())
}
