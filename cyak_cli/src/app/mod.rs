pub mod error;

mod ui;

pub use error::Error;

use crate::cli::{Cli, PresetPath, SubCommand};

use ui::Ui;

use cyak_core::ProjectConfig;
use cyak_core::{Context, PresetConfig};

use fs_extra::dir::CopyOptions;

use std::cell::RefCell;
use std::path::PathBuf;
use std::rc::Rc;

pub fn run(cli: Cli) -> anyhow::Result<()> {
    // If share dir not exists
    if !cli.share_dir.is_dir() {
        return Error::DirNotFound(cli.share_dir).anyhow_fail();
    }

    match cli.subcommand {
        SubCommand::New(_) => new_project(cli),
        SubCommand::Install(_) => install_preset(cli),
    }
}

fn new_project(cli: Cli) -> anyhow::Result<()> {
    let ui = {
        let project_dir = match cli.subcommand {
            SubCommand::New(c) => c.project_dir,
            _ => return Error::InvalidCliSubCommand("new".to_string()).anyhow_fail(),
        };

        Ui::new(&cli.share_dir, &project_dir)?
    };

    let mut siv = cursive::default();

    siv.run();

    Ok(())
}

fn install_preset(cli: Cli) -> anyhow::Result<()> {
    let path = match cli.subcommand {
        SubCommand::Install(c) => c.preset_path,
        _ => return Error::InvalidCliSubCommand("install".to_string()).anyhow_fail(),
    };

    match path {
        PresetPath::Local(p) => {
            let in_dir = PathBuf::from(p);
            // If source dir not exists
            if !in_dir.is_dir() {
                return Error::DirNotFound(in_dir).anyhow_fail();
            }

            let in_dir = in_dir.canonicalize()?;
            let in_dir_name = in_dir
                .file_name()
                .ok_or_else(|| Error::DirNotFound(in_dir.clone()))?;

            let out_dir = cli.share_dir.join("presets");
            let out_dir_full = out_dir.join(in_dir_name);
            // If destination dir exists
            if out_dir_full.is_dir() {
                return Error::DirExists(out_dir_full).anyhow_fail();
            }

            // Copy preset dir
            {
                let opts = CopyOptions::new();
                fs_extra::dir::copy(&in_dir, &out_dir, &opts)?;
            }
        }
        PresetPath::Url(p) => {
            let &rep_name = p
                .path_segments()
                .map(|c| c.collect::<Vec<_>>())
                .ok_or_else(|| Error::InvalidUrl("no URL segments".to_string()))?
                .iter()
                .last()
                .ok_or_else(|| {
                    Error::InvalidUrl("git repository name from URL not found".to_string())
                })?;

            let out_dir = cli.share_dir.join("presets").join(rep_name);

            git2::Repository::clone(p.as_str(), &out_dir)?;
        }
    }
    Ok(())
}
