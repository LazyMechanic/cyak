pub mod error;

pub use error::*;

use clap::AppSettings;
use std::env;
use std::path::PathBuf;

const APP_NAME: &str = "cyak";
const APP_VERSION: &str = "0.6.0";
const APP_AUTHOR: &str = "LazyMechanic <asharnrus@gmail.com>";
const APP_ABOUT: &str = "Tool for create new or modify exists cmake project";

const SHARE_DIR_NAME: &str = "share directory";
const SHARE_DIR_LONG: &str = "share-dir";
const SHARE_DIR_SHORT: &str = "s";
const SHARE_DIR_HELP: &str = "Directory with share data";
const SHARE_DIR_VALUE_NAME: &str = "PATH";

const PATH_NAME: &str = "PATH";
const PATH_HELP: &str = "Path to project";

#[derive(Debug)]
pub struct Cli {
    pub share_dir: PathBuf,
    pub work_dir: PathBuf,
}

impl Cli {
    pub fn new() -> Result<Self, Error> {
        let default_share_dir = {
            let base = if cfg!(windows) {
                let current_dir = env::current_dir()?;
                let base = current_dir.as_path();
                let base = base.parent().ok_or_else(|| Error::ShareDataDirNotFound)?;
                base.join("share")
            } else {
                let base = env::var("XDG_DATA_HOME")?;
                PathBuf::from(base).join("cyak").join("share")
            };

            let dir = PathBuf::from("cyak");
            base.join(dir)
        };

        let app = clap::App::new(APP_NAME)
            .version(APP_VERSION)
            .author(APP_AUTHOR)
            .about(APP_ABOUT)
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
            .setting(AppSettings::ArgRequiredElseHelp);

        let matches = app.get_matches();

        let share_dir = matches
            .value_of(SHARE_DIR_NAME)
            .ok_or_else(|| Error::ArgumentNotFound(SHARE_DIR_LONG.to_string()))?;
        let share_dir = PathBuf::from(share_dir);

        let work_dir = matches
            .value_of(PATH_NAME)
            .ok_or_else(|| Error::ArgumentNotFound(PATH_NAME.to_string()))?;
        let work_dir = PathBuf::from(work_dir);

        Ok(Self {
            work_dir,
            share_dir,
        })
    }
}
