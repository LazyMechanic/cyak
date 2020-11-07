pub mod menu;

pub use menu::Menu;

use cyak_core::{Context, PresetConfig};
use std::path::PathBuf;

pub struct Ui {
    pub ctx: Context,
    pub share_dir: PathBuf,
    pub preset_config: PresetConfig,
}
