use cyak_core::Error;

mod common;

#[test]
fn validate_valid_preset() -> anyhow::Result<()> {
    let preset_dir = common::create_mock_preset()?;
    assert_eq!(cyak_core::validate_preset(&preset_dir), Ok(()));
    Ok(())
}

#[test]
fn validate_invalid_preset() -> anyhow::Result<()> {
    let preset_dir = common::create_mock_invalid_preset()?;
    assert_ne!(cyak_core::validate_preset(&preset_dir), Ok(()));
    Ok(())
}
