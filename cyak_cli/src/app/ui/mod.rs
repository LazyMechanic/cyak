pub mod error;
pub mod menu;
pub mod views;

pub use error::*;
pub use menu::*;
pub use views::*;

use cyak_core::{Context, PresetConfig};
use std::path::PathBuf;

pub struct Ui {
    pub ctx: Context,
    pub share_dir: PathBuf,
    pub preset_config: PresetConfig,
}

pub const DEFAULT_VIEW_SIZE: (usize, usize) = (30, 15);
