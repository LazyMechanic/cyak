use config::{Config, File, FileFormat};
use serde::Deserialize;

use crate::cli::Error;

#[derive(Debug, Clone, Deserialize)]
pub struct Settings {
    pub name: String,
    pub version: String,
    pub author: String,
    pub about: String,
}

impl Settings {
    pub fn new() -> Result<Self, Error> {
        let mut config = Config::new();
        config.merge(File::from_str(
            include_str!("settings.yaml"),
            FileFormat::Yaml,
        ))?;

        let settings = config.try_into()?;
        Ok(settings)
    }
}
