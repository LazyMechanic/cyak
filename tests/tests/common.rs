use std::path::{Path, PathBuf};

use uuid::Uuid;

pub const STUFF_DIR: &str = "tests/stuff";

pub fn finalize_path<P: AsRef<Path>>(path: P) -> PathBuf {
    let dir = PathBuf::from(STUFF_DIR);
    dir.join(path)
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct DirGuard(PathBuf);

impl DirGuard {
    pub fn new<P: AsRef<Path>>(path: P) -> Self {
        Self(path.as_ref().to_path_buf())
    }
}

impl Drop for DirGuard {
    fn drop(&mut self) {
        match self.0.exists() {
            true => std::fs::remove_dir_all(&self.0).unwrap(),
            false => { /*do nothing*/ }
        }
    }
}

#[test]
fn check_dir_guard() -> anyhow::Result<()> {
    let dir = Uuid::new_v4().to_string();
    let dir_to_create = finalize_path(&dir);
    {
        std::fs::create_dir(&dir_to_create)?;
        let _guard = DirGuard::new(&dir_to_create);
    }

    assert!(!dir_to_create.exists());

    Ok(())
}
