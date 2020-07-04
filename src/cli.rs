use clap::{App, AppSettings, Arg, ArgMatches};
use std::path::PathBuf;

use crate::cli::Error::ArgumentNotFound;
use cyak_lib::config;

#[derive(Debug)]
pub struct Cli {
    pub command: Command,
}

#[derive(Debug)]
pub enum Command {
    New(New),
    Modify(Modify),
}

#[derive(Debug)]
pub struct New {
    pub path: PathBuf,
}

#[derive(Debug)]
pub struct Modify {
    pub path: PathBuf,
}

impl Cli {
    pub fn new() -> Result<Self, Error> {
        let app = App::new(config::PRG_NAME)
            .version(config::PRG_VERSION)
            .author(config::PRG_AUTHOR)
            .about(config::PRG_HELP)
            .subcommand(
                App::new("new").about("Create new cmake project").arg(
                    Arg::with_name("PATH")
                        .help("Path to project")
                        .required(true),
                ),
            )
            .subcommand(
                App::new("modify").about("Modify exist cmake project").arg(
                    Arg::with_name("PATH")
                        .help("Path to project")
                        .required(true),
                ),
            )
            .setting(AppSettings::ArgRequiredElseHelp);

        let matches = app.get_matches();
        match matches.subcommand() {
            ("new", Some(c)) => Self::new_cmd_from_args(c),
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

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("Invalid cli subcommand")]
    InvalidSubCommand,
    #[error("Argument not found: {0}")]
    ArgumentNotFound(String),
}

impl Error {
    pub fn fail<T>(self) -> std::result::Result<T, Self> {
        std::result::Result::Err(self)
    }
}
