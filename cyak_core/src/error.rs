use std::path::PathBuf;

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("Project already generated: {0}")]
    ProjectAlreadyGenerated(PathBuf),
    #[error("Invalid preset structure: {}",
    .0
    .iter()
    .fold(String::new(), |res, (msg, file)| res + format!("\n- {} {:?}", msg, file).as_str()))]
    InvalidPresetStructure(Vec<(String, PathBuf)>),
    #[error("Directory not found: {0:?}")]
    DirNotFound(PathBuf),
    #[error("Invalid file name: {0:?}")]
    InvalidFilename(PathBuf),
    #[error("Path is not a directory: {0:?}")]
    NotDir(PathBuf),
    #[error("Path is not a file: {0:?}")]
    NotFile(PathBuf),
    #[error(transparent)]
    IoError(#[from] std::io::Error),
    #[error(transparent)]
    SerdeYamlError(#[from] serde_yaml::Error),
    #[error(transparent)]
    FsExtraError(#[from] fs_extra::error::Error),
}

impl Error {
    pub fn fail<T>(self) -> std::result::Result<T, Self> {
        std::result::Result::Err(self)
    }
}
