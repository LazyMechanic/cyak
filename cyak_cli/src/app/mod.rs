pub mod error;

pub use error::Error;

use crate::cli;
use crate::cli::{Cli, SubCommand};

use cyak_core::PresetConfig;
use cyak_core::ProjectConfig;

pub fn run(cli: Cli) -> anyhow::Result<()> {
    match cli.subcommand {
        SubCommand::New(_) => new_project(cli),
        SubCommand::Install(_) => install_preset(cli),
    }
}

fn new_project(cli: Cli) -> anyhow::Result<()> {
    cyak_core::utils::check_dir_existence(&cli.share_dir)?;

    let preset_dir = cli.share_dir.join("presets").join("default");
    cyak_core::utils::check_dir_existence(&preset_dir)?;

    let preset_config = cyak_core::load_preset_config(&preset_dir)?;

    let project_dir = match cli.subcommand {
        SubCommand::New(c) => c.project_dir,
        _ => return Error::InvalidCliSubCommand("new".to_string()).anyhow_fail(),
    };
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

fn install_preset(cli: Cli) -> anyhow::Result<()> {
    Ok(())
}
