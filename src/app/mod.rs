pub mod cli;
pub mod tui;

use std::path::PathBuf;

use self::cli::Cli;
use self::cli::Command;
use self::tui::Tui;

pub const APP_NAME: &'static str = "cyak";
pub const APP_VERSION: &'static str = "0.6.0";
pub const APP_AUTHOR: &'static str = "LazyMechanic <asharnrus@gmail.com>";
pub const APP_ABOUT: &'static str = "Tool for create new or modify exists cmake project";

pub struct App {
    user_interface: UserInterface,
    path:           PathBuf,
}

enum UserInterface {
    Tui(Tui),
    Gui(),
}

impl UserInterface {
    pub fn run(&mut self) -> anyhow::Result<()> {
        match self {
            UserInterface::Tui(i) => i.run()?,
            UserInterface::Gui() => todo!(),
        }

        Ok(())
    }
}

impl App {
    pub fn new() -> anyhow::Result<Self> {
        let cli = cli::Cli::new()?;
        match cli.command {
            Command::New(c) => {
                let tui = Tui::new()?;
                Ok(Self {
                    user_interface: UserInterface::Tui(tui),
                    path:           c.path,
                })
            }
            Command::Modify(_) => todo!(),
            Command::Gui(_) => todo!(),
        }
    }

    pub fn run(&mut self) -> anyhow::Result<()> {
        self.user_interface.run()?;
        Ok(())
    }
}
