use std::path::{Path, PathBuf};

pub const STUFF_DIR: &str = "tests/stuff";

pub fn finalize_path<P: AsRef<Path>>(p: P) -> PathBuf {
    let dir = PathBuf::from(STUFF_DIR);
    let dir = dir.join(p);
    std::fs::canonicalize(dir).expect("finalized path not found")
}
