use clap::{AppSettings, Arg, ArgMatches};
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
const GUI_ABOUT: &str = "Start in gui mod (NOT SUPPORTED YET)";

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
}

impl Cli {
    pub fn new() -> Result<Self, Error> {
        let app = clap::App::new(APP_NAME)
            .version(APP_VERSION)
            .author(APP_AUTHOR)
            .about(APP_ABOUT)
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
            (MODIFY_NAME, Some(c)) => Self::modify_cmd_from_args(c),
            (GUI_NAME, Some(c)) => Self::gui_cmd_from_args(c),
            _ => Error::InvalidSubCommand.fail(),
        }
    }

    fn new_cmd_from_args(am: &ArgMatches) -> Result<Self, Error> {
        let path = am
            .value_of("PATH")
            .ok_or(Error::ArgumentNotFound("PATH".to_string()))?;

        Ok(Self {
            command: Command::New(New::new(path)),
        })
    }

    fn modify_cmd_from_args(am: &ArgMatches) -> Result<Self, Error> {
        Error::UnsupportedSubCommand.fail()
    }

    fn gui_cmd_from_args(am: &ArgMatches) -> Result<Self, Error> {
        Error::UnsupportedSubCommand.fail()
    }
}
