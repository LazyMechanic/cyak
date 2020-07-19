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
    #[error(transparent)]
    IoError(#[from] std::io::Error),
}

impl Error {
    pub fn fail<T>(self) -> std::result::Result<T, Self> {
        std::result::Result::Err(self)
    }
}
