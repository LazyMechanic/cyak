use std::path::PathBuf;

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("Project directory exists, but cyak config file not found: {0:?}")]
    ProjectDirExists(PathBuf),
}

impl Error {
    pub fn fail<T>(self) -> std::result::Result<T, Self> {
        std::result::Result::Err(self)
    }

    pub fn anyhow_fail<T>(self) -> anyhow::Result<T> {
        Err(anyhow::anyhow!(self))
    }
}
