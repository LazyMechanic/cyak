#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("Invalid cli subcommand")]
    InvalidSubCommand,
    #[error("Argument not found: {0}")]
    ArgumentNotFound(String),
    #[error("Unsupported cli subcommand")]
    UnsupportedSubCommand,
    #[error(transparent)]
    EnvVarError(#[from] std::env::VarError),
    #[error("Presets directory not found")]
    PresetsDirNotFound,
    #[error(transparent)]
    IoError(#[from] std::io::Error),
}

impl Error {
    pub fn fail<T>(self) -> std::result::Result<T, Self> {
        std::result::Result::Err(self)
    }
}
