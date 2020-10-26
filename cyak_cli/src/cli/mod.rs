pub mod error;
pub mod settings;

pub use error::*;

use clap::AppSettings;
use std::env;
use std::path::PathBuf;
use url::Url;

const SHARE_DIR_OPTION_NAME: &str = "share directory";
const SHARE_DIR_OPTION_LONG: &str = "share-dir";
const SHARE_DIR_OPTION_SHORT: &str = "s";
const SHARE_DIR_OPTION_HELP: &str = "Directory with share data";
const SHARE_DIR_OPTION_VALUE_NAME: &str = "PATH";

const NEW_CMD_NAME: &str = "new";
const NEW_CMD_ABOUT: &str = "Create new project";

const PROJECT_PATH_ARG_NAME: &str = "PROJECT_PATH";
const PROJECT_PATH_ARG_HELP: &str = "Path to project";

const INSTALL_CMD_NAME: &str = "install";
const INSTALL_CMD_ABOUT: &str = "Install preset from local directory or from URL by git. For example: '/home/user/custom_preset' or 'https://github.com/user/preset'";

const INSTALL_PRESET_PATH_ARG_NAME: &str = "PRESET_PATH";
const INSTALL_PRESET_PATH_ARG_HELP: &str = "Preset local path or URL";

#[derive(Debug)]
pub enum PresetPath {
    Local(String),
    Url(Url),
}

#[derive(Debug)]
pub struct New {
    pub project_dir: PathBuf,
}

#[derive(Debug)]
pub struct Install {
    pub preset_dir: PresetPath,
}

pub enum SubCommand {
    New(New),
    Install(Install),
}

pub struct Cli {
    pub share_dir: PathBuf,
    pub subcommand: SubCommand,
}

impl Cli {
    pub fn new() -> Result<Self, Error> {
        let default_share_dir = {
            let base = if cfg!(windows) {
                let bin = env::current_exe()?;
                let mut dir = bin
                    .parent()
                    .ok_or_else(|| Error::NoParentDir(bin.clone()))?
                    .to_path_buf();
                dir.pop();
                dir
            } else {
                PathBuf::from(env::var("XDG_DATA_HOME")?).join("cyak")
            };

            base.join("share")
        };

        let settings = settings::Settings::new()?;

        let app = clap::App::new(settings.name.as_str())
            .version(settings.version.as_str())
            .author(settings.author.as_str())
            .about(settings.about.as_str())
            .arg(
                clap::Arg::with_name(SHARE_DIR_OPTION_NAME)
                    .long(SHARE_DIR_OPTION_LONG)
                    .short(SHARE_DIR_OPTION_SHORT)
                    .help(SHARE_DIR_OPTION_HELP)
                    .value_name(SHARE_DIR_OPTION_VALUE_NAME)
                    .default_value_os(default_share_dir.as_os_str()),
            )
            .subcommand(
                clap::App::new(NEW_CMD_NAME).about(NEW_CMD_ABOUT).arg(
                    clap::Arg::with_name(PROJECT_PATH_ARG_NAME)
                        .help(PROJECT_PATH_ARG_HELP)
                        .required(true),
                ),
            )
            .subcommand(
                clap::App::new(INSTALL_CMD_NAME)
                    .about(INSTALL_CMD_ABOUT)
                    .arg(
                        clap::Arg::with_name(INSTALL_PRESET_PATH_ARG_NAME)
                            .help(INSTALL_PRESET_PATH_ARG_HELP)
                            .required(true),
                    ),
            )
            .setting(AppSettings::ArgRequiredElseHelp);

        // Global options
        let matches = app.get_matches();
        let share_dir = matches
            .value_of(SHARE_DIR_OPTION_NAME)
            .ok_or_else(|| Error::ArgumentNotFound(SHARE_DIR_OPTION_LONG.to_string()))?;
        let share_dir = PathBuf::from(share_dir);

        let cli = match matches.subcommand() {
            (INSTALL_CMD_NAME, Some(s)) => {
                let preset_dir = s.value_of(INSTALL_PRESET_PATH_ARG_NAME).ok_or_else(|| {
                    Error::ArgumentNotFound(INSTALL_PRESET_PATH_ARG_NAME.to_string())
                })?;

                let preset_dir = if let Ok(u) = Url::parse(preset_dir) {
                    PresetPath::Url(u)
                } else {
                    PresetPath::Local(preset_dir.to_string())
                };

                Cli {
                    share_dir,
                    subcommand: SubCommand::Install(Install { preset_dir }),
                }
            }
            (NEW_CMD_NAME, Some(s)) => {
                let project_dir = s
                    .value_of(PROJECT_PATH_ARG_NAME)
                    .ok_or_else(|| Error::ArgumentNotFound(PROJECT_PATH_ARG_NAME.to_string()))?;
                let project_dir = PathBuf::from(project_dir);

                Cli {
                    share_dir,
                    subcommand: SubCommand::New(New { project_dir }),
                }
            }
            _ => unreachable!(),
        };

        Ok(cli)
    }
}
