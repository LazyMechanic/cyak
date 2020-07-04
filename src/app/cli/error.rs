#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("Invalid cli subcommand")]
    InvalidSubCommand,
    #[error("Argument not found: {0}")]
    ArgumentNotFound(String),
}

impl Error {
    pub fn fail<T>(self) -> std::result::Result<T, Self> {
        std::result::Result::Err(self)
    }
}
