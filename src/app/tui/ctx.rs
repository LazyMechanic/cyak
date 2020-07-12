use std::path::PathBuf;

#[derive(Debug)]
pub struct Context {
    pub is_running:     bool,
    pub share_data_dir: PathBuf,
    pub work_dir:       PathBuf,
}

impl Context {
    pub fn new(share_data_dir: PathBuf, work_dir: PathBuf) -> Self {
        Self {
            is_running: true,
            share_data_dir,
            work_dir,
        }
    }
}
