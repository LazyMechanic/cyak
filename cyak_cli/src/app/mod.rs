pub mod error;

mod traits;
mod ui;

pub use error::Error;

use crate::cli;
use crate::cli::{Cli, PresetPath, SubCommand};

use cursive::align::HAlign;
use cursive::traits::Resizable;
use cursive::view::{Nameable, Scrollable};
use cursive::views::{Dialog, SelectView, TextView, ViewRef};
use cursive::{Cursive, With};
use cyak_core::context::Context;
use cyak_core::PresetConfig;
use cyak_core::ProjectConfig;
use fs_extra::dir::{CopyOptions, DirOptions};
use std::cell::RefCell;
use std::ops::Deref;
use std::path::PathBuf;
use std::rc::Rc;
use std::sync::Arc;

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
    let project_dir = match cli.subcommand {
        SubCommand::New(c) => c.project_dir,
        _ => return Error::InvalidCliSubCommand("new".to_string()).anyhow_fail(),
    };

    cyak_core::utils::check_dir_existence(&cli.share_dir)?;

    let preset_dir = cli.share_dir.join("presets").join("default");
    cyak_core::utils::check_dir_existence(&preset_dir)?;

    let preset_config = cyak_core::load_preset_config(&preset_dir)?;

    // If project dir already exist
    if project_dir.exists() {
        return Error::ProjectDirExists(project_dir.clone()).anyhow_fail();
    }

    let project_config = ProjectConfig::default();

    // ************************************************************************************** //

    let ctx = Rc::new(RefCell::new(Context {
        project_dir,
        preset_dir,
        git: false,
        license: None,
        project_config,
    }));

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
                let mut opts = CopyOptions::new();
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
