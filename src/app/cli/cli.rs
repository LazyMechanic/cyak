use clap::{AppSettings, Arg, ArgMatches};
use std::env;
use std::path::PathBuf;

use super::Error;

const APP_NAME: &'static str = "cyak";
const APP_VERSION: &'static str = "0.6.0";
const APP_AUTHOR: &'static str = "LazyMechanic <asharnrus@gmail.com>";
const APP_ABOUT: &'static str = "Tool for create new or modify exists cmake project";

const NEW_NAME: &str = "new";
const NEW_ABOUT: &str = "Create new cmake project";

const MODIFY_NAME: &str = "modify";
const MODIFY_ABOUT: &str = "Modify exists cmake project (NOT SUPPORTED YET)";

const GUI_NAME: &str = "gui";
const GUI_LONG: &str = "gui";
const GUI_SHORT: &str = "g";
const GUI_HELP: &str = "Start in gui mod (NOT SUPPORTED YET)";

const SHARE_DATA_NAME: &str = "share data dir";
const SHARE_DATA_LONG: &str = "share-data-dir";
const SHARE_DATA_SHORT: &str = "s";
const SHARE_DATA_HELP: &str = "Directory with share data";
const SHARE_DATA_VALUE_NAME: &str = "PATH";

const PATH_NAME: &str = "PATH";
const PATH_HELP: &str = "Path to project, required in TUI mode";

#[derive(Debug)]
pub struct Cli {
    pub share_data_dir: PathBuf,
    pub work_dir:       PathBuf,
    pub mode:           Mode,
}

#[derive(Debug, Eq, PartialEq)]
pub enum Mode {
    Tui,
    Gui,
}

impl Cli {
    pub fn new() -> Result<Self, Error> {
        let default_share_data_dir = {
            let base = if cfg!(windows) {
                let current_dir = env::current_dir()?;
                let base = current_dir.as_path();
                let base = base.parent().ok_or(Error::ShareDataDirNotFound)?;
                let base = base.join("share");
                base
            } else {
                let base = env::var("XDG_DATA_HOME")?;
                let base = PathBuf::from(base).join("cyak").join("share");
                base
            };

            let dir = PathBuf::from("cyak");
            let full = base.join(dir);
            full
        };

        let app = clap::App::new(APP_NAME)
            .version(APP_VERSION)
            .author(APP_AUTHOR)
            .about(APP_ABOUT)
            .arg(
                clap::Arg::with_name(SHARE_DATA_NAME)
                    .long(SHARE_DATA_LONG)
                    .short(SHARE_DATA_SHORT)
                    .help(SHARE_DATA_HELP)
                    .value_name(SHARE_DATA_VALUE_NAME)
                    .default_value_os(default_share_data_dir.as_os_str()),
            )
            .arg(
                clap::Arg::with_name(GUI_NAME)
                    .long(GUI_LONG)
                    .short(GUI_SHORT)
                    .help(GUI_HELP),
            )
            .arg(
                clap::Arg::with_name(PATH_NAME)
                    .help(PATH_HELP)
                    .required(false),
            )
            .setting(AppSettings::ArgRequiredElseHelp);

        let matches = app.get_matches();

        let share_data_dir = matches
            .value_of(SHARE_DATA_NAME)
            .ok_or(Error::ArgumentNotFound(SHARE_DATA_LONG.to_string()))?;
        let share_data_dir = PathBuf::from(share_data_dir);

        let mode = if matches.occurrences_of(GUI_NAME) >= 1 {
            Mode::Gui
        } else {
            Mode::Tui
        };

        let work_dir = matches
            .value_of(PATH_NAME)
            .or_else(|| if mode == Mode::Gui { Some("") } else { None })
            .ok_or(Error::ArgumentNotFound(PATH_NAME.to_string()))?;
        let work_dir = PathBuf::from(work_dir);

        Ok(Self {
            work_dir,
            share_data_dir,
            mode,
        })
    }
}
