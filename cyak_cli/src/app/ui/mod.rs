pub mod error;
pub mod menu;

pub use error::*;
pub use menu::*;

use cyak_core::{Context, PresetConfig};
use std::path::PathBuf;

pub struct Ui {
    pub ctx: Context,
    pub share_dir: PathBuf,
    pub preset_config: PresetConfig,
}
