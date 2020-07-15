use cyak_core::lang::Language;
use cyak_core::project_config::ProjectConfig;
use cyak_core::version::Version;
use std::fs;
use std::path::PathBuf;

mod common;

const ALREADY_GENERATED_PROJECT: &str = "already_generated";

#[test]
fn check_already_generated_project() {
    let dir = common::finalize_path(ALREADY_GENERATED_PROJECT);
    assert!(cyak_core::is_project_already_generated(dir));
}
