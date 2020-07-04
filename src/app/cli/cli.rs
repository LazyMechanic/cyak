use clap::{AppSettings, Arg, ArgMatches};
use std::path::PathBuf;

use super::Error;
use crate::app;

const NEW_NAME: &str = "new";
const NEW_ABOUT: &str = "Create new cmake project";

const MODIFY_NAME: &str = "modify";
const MODIFY_ABOUT: &str = "Modify exists cmake project";

const GUI_NAME: &str = "gui";
const GUI_ABOUT: &str = "Start in gui mod";

#[derive(Debug)]
pub struct Cli {
    pub command: Command,
}

#[derive(Debug)]
pub enum Command {
    New(New),
    Modify(Modify),
    Gui(Gui),
}

#[derive(Debug)]
pub struct New {
    pub path: PathBuf,
}

impl New {
    pub fn new<T: Into<PathBuf>>(path: T) -> Self {
        Self { path: path.into() }
    }
}

#[derive(Debug)]
pub struct Modify {
    pub path: PathBuf,
}

impl Modify {
    pub fn new<T: Into<PathBuf>>(path: T) -> Self {
        Self { path: path.into() }
    }
}

#[derive(Debug)]
pub struct Gui {}

impl Gui {
    pub fn new() -> Self {
        Self {}
    }

    pub fn name() -> String {
        "gui".to_string()
    }

    pub fn about() -> String {
        "Start in gui mod".to_string()
    }
}

impl Cli {
    pub fn new() -> Result<Self, Error> {
        let app = clap::App::new(app::APP_NAME)
            .version(app::APP_VERSION)
            .author(app::APP_AUTHOR)
            .about(app::APP_ABOUT)
            .subcommand(
                clap::App::new(NEW_NAME).about(NEW_ABOUT).arg(
                    Arg::with_name("PATH")
                        .help("Path to project")
                        .required(true),
                ),
            )
            .subcommand(
                clap::App::new(MODIFY_NAME).about(MODIFY_ABOUT).arg(
                    Arg::with_name("PATH")
                        .help("Path to project")
                        .required(true),
                ),
            )
            .subcommand(clap::App::new(GUI_NAME).about(GUI_ABOUT))
            .setting(AppSettings::ArgRequiredElseHelp);

        let matches = app.get_matches();
        match matches.subcommand() {
            (NEW_NAME, Some(c)) => Self::new_cmd_from_args(c),
            (MODIFY_NAME, Some(c)) => todo!(),
            (GUI_NAME, Some(c)) => todo!(),
            _ => Error::InvalidSubCommand.fail(),
        }
    }

    fn new_cmd_from_args(am: &ArgMatches) -> Result<Self, Error> {
        let path = am
            .value_of("PATH")
            .ok_or(Error::ArgumentNotFound("PATH".to_string()))?;

        let path = PathBuf::from(path);

        Ok(Self {
            command: Command::New(New { path }),
        })
    }
}
