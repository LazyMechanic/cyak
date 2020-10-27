use std::ffi::OsString;
use std::path::PathBuf;

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("Project directory exists, but cyak config file not found: {0:?}")]
    ProjectDirExists(PathBuf),
    #[error("Directory not found: {0:?}")]
    DirNotFound(PathBuf),
    #[error("Directory already exists: {0:?}")]
    DirExists(PathBuf),
    #[error("Invalid cli sub command, expected: {0}")]
    InvalidCliSubCommand(String),
    #[error("Invalid URL: {0}")]
    InvalidUrl(String),
    #[error("Cannot convert &OsStr to &str: {0:?}")]
    ConvertOsStrToStr(OsString),
    #[error(transparent)]
    GitError(#[from] git2::Error),
    #[error(transparent)]
    IoError(#[from] std::io::Error),
}

impl Error {
    pub fn fail<T>(self) -> std::result::Result<T, Self> {
        std::result::Result::Err(self)
    }

    pub fn anyhow_fail<T>(self) -> anyhow::Result<T> {
        Err(anyhow::anyhow!(self))
    }
}
