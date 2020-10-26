#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("Argument not found: {0}")]
    ArgumentNotFound(String),
    #[error("No parent directory: {0}")]
    NoParentDir(std::path::PathBuf),
    #[error(transparent)]
    EnvVarError(#[from] std::env::VarError),
    #[error(transparent)]
    IoError(#[from] std::io::Error),
    #[error(transparent)]
    ConfigError(#[from] config::ConfigError),
}

impl Error {
    pub fn fail<T>(self) -> std::result::Result<T, Self> {
        std::result::Result::Err(self)
    }
}
