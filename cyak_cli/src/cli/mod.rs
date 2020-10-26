pub mod error;
pub mod settings;

pub use error::*;

use clap::AppSettings;
use std::env;
use std::path::PathBuf;
use url::Url;

const PATH_NAME: &str = "PROJECT_PATH";
const PATH_HELP: &str = "Path to project";

const SHARE_DIR_NAME: &str = "share directory";
const SHARE_DIR_LONG: &str = "share-dir";
const SHARE_DIR_SHORT: &str = "s";
const SHARE_DIR_HELP: &str = "Directory with share data";
const SHARE_DIR_VALUE_NAME: &str = "PATH";

const INSTALL_NAME: &str = "install";
const INSTALL_ABOUT: &str = "Install preset from local directory or from URL by git. For example: '/home/user/custom_preset' or 'https://github.com/user/preset'";

const INSTALL_PRESET_PATH_NAME: &str = "PRESET_PATH";
const INSTALL_PRESET_PATH_HELP: &str = "Preset local path or URL";

#[derive(Debug)]
pub enum PresetPath {
    Local(String),
    Url(Url),
}

#[derive(Debug)]
pub struct Create {
    pub share_dir: PathBuf,
    pub project_dir: PathBuf,
}

#[derive(Debug)]
pub struct Install {
    pub preset_dir: PresetPath,
}

pub enum SubCommand {
    Create(Create),
    Install(Install),
}

pub struct Cli(pub SubCommand);

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
                clap::Arg::with_name(SHARE_DIR_NAME)
                    .long(SHARE_DIR_LONG)
                    .short(SHARE_DIR_SHORT)
                    .help(SHARE_DIR_HELP)
                    .value_name(SHARE_DIR_VALUE_NAME)
                    .default_value_os(default_share_dir.as_os_str()),
            )
            .arg(
                clap::Arg::with_name(PATH_NAME)
                    .help(PATH_HELP)
                    .required(false),
            )
            .subcommand(
                clap::App::new("install")
                    .arg(
                        clap::Arg::with_name(INSTALL_PRESET_PATH_NAME)
                            .help(INSTALL_PRESET_PATH_HELP)
                            .required(true),
                    )
                    .about(INSTALL_ABOUT),
            )
            .setting(AppSettings::ArgRequiredElseHelp);

        let matches = app.get_matches();

        let cli = match matches.subcommand() {
            (INSTALL_NAME, Some(s)) => {
                let preset_dir = s
                    .value_of(INSTALL_PRESET_PATH_NAME)
                    .ok_or_else(|| Error::ArgumentNotFound(INSTALL_PRESET_PATH_NAME.to_string()))?;

                let preset_dir = if let Ok(u) = Url::parse(preset_dir) {
                    PresetPath::Url(u)
                } else {
                    PresetPath::Local(preset_dir.to_string())
                };

                Cli(SubCommand::Install(Install { preset_dir }))
            }
            _ => {
                let share_dir = matches
                    .value_of(SHARE_DIR_NAME)
                    .ok_or_else(|| Error::ArgumentNotFound(SHARE_DIR_LONG.to_string()))?;
                let share_dir = PathBuf::from(share_dir);

                let project_dir = matches
                    .value_of(PATH_NAME)
                    .ok_or_else(|| Error::ArgumentNotFound(PATH_NAME.to_string()))?;
                let project_dir = PathBuf::from(project_dir);

                Cli(SubCommand::Create(Create {
                    share_dir,
                    project_dir,
                }))
            }
        };

        Ok(cli)
    }
}
