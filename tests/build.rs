use std::fs;
use std::path::PathBuf;

pub const STUFF_DIR: &str = "tests/stuff";

fn main() {
    let stuff_dir = PathBuf::from(STUFF_DIR);
    if stuff_dir.exists() {
        fs::remove_dir_all(&stuff_dir).unwrap();
    }

    fs::create_dir_all(&stuff_dir).unwrap();
}
