use std::path::PathBuf;

#[derive(thiserror::Error, Debug, PartialEq, Eq)]
pub enum Error {
    #[error("Project already generated: {0}")]
    ProjectAlreadyGenerated(PathBuf),
    #[error("Invalid preset structure: {}",
    .0
    .iter()
    .fold(String::new(), |res, f| res + format!("\n- missing {:?}", f).as_str()))]
    InvalidPresetStructure(Vec<PathBuf>),
}

impl Error {
    pub fn fail<T>(self) -> std::result::Result<T, Self> {
        std::result::Result::Err(self)
    }
}
