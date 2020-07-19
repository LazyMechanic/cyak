use cyak_core::Error;

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
