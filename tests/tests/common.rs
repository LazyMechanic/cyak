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
    let project_dir_path = finalize_path(&Uuid::new_v4().to_string());
    let config_dir_path = project_dir_path.join(cyak_core::PROJECT_CONFIG_DIR);
    let file_path = config_dir_path.join(cyak_core::PROJECT_CONFIG_FILE);

    fs::create_dir_all(&config_dir_path)?;
    File::create(file_path)?;

    Ok(project_dir_path)
}
