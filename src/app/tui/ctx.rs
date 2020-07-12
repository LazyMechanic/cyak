use std::path::PathBuf;

#[derive(Debug)]
pub struct Context {
    pub is_running:  bool,
    pub presets_dir: PathBuf,
    pub work_dir:    PathBuf,
}

impl Context {
    pub fn new(presets_dir: PathBuf, work_dir: PathBuf) -> Self {
        Self {
            is_running: true,
            presets_dir,
            work_dir,
        }
    }
}
