#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error(transparent)]
    IoError(#[from] std::io::Error),
    #[error(transparent)]
    CrosstermError(#[from] crossterm::ErrorKind),
    #[error("Terminal not initialized. Call '.init(...)' before use it")]
    TerminalNotInitialized,
    #[error(transparent)]
    MpscRecvError(#[from] std::sync::mpsc::RecvError),
}

impl Error {
    pub fn fail<T>(self) -> std::result::Result<T, Self> {
        std::result::Result::Err(self)
    }
}
