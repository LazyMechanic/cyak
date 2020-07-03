use clap::{App, AppSettings, Arg};
use std::path::PathBuf;

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
        let mut app = App::new("cyak")
            .version("0.6")
            .author("LazyMechanic <asharnrus@gmail.com>")
            .about("Tool for create new or modify exists cmake project")
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
            ("new", Some(c)) => {
                let path = c.value_of("PATH").unwrap();
                Ok(Self {
                    command: Command::New(New {
                        path: PathBuf::from(path),
                    }),
                })
            }
            _ => Error::InvalidSubCommand.fail(),
        }
    }
}

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("Invalid cli subcommand")]
    InvalidSubCommand,
}

impl Error {
    pub fn fail<T>(self) -> std::result::Result<T, Self> {
        std::result::Result::Err(self)
    }
}
