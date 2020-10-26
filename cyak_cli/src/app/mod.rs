pub mod error;

pub use error::Error;

use crate::cli;
use crate::cli::{Cli, SubCommand};

use cyak_core::PresetConfig;
use cyak_core::ProjectConfig;

pub fn run(cli: Cli) -> anyhow::Result<()> {
    match cli.0 {
        SubCommand::Create(c) => create_project(c),
        SubCommand::Install(c) => install_preset(c),
    }
}

fn install_preset(c: cli::Install) -> anyhow::Result<()> {
    Ok(())
}

fn create_project(c: cli::Create) -> anyhow::Result<()> {
    cyak_core::utils::check_dir_existence(&c.share_dir)?;

    let preset_dir = c.share_dir.join("presets").join("default");
    cyak_core::utils::check_dir_existence(&preset_dir)?;

    let preset_config = cyak_core::load_preset_config(&preset_dir)?;

    let project_dir = c.project_dir;
    // If project dir already exist
    if project_dir.exists() {
        return Error::ProjectDirExists(project_dir.clone()).anyhow_fail();
    }

    let project_config = ProjectConfig::default();

    let mut siv = cursive::default();

    siv.add_global_callback('q', |s| s.quit());

    siv.add_layer(cursive::views::TextView::new(
        "Hello cursive! Press <q> to quit.",
    ));

    siv.run();

    Ok(())
}
