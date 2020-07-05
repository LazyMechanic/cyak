pub mod cli;
pub mod tui;

use std::path::PathBuf;

use self::cli::Cli;
use self::cli::Command;
use self::tui::Tui;

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

pub fn run() -> anyhow::Result<()> {
    let cli = cli::Cli::new()?;

    let mut ui = match cli.command {
        Command::New(c) => {
            let tui = Tui::new(c.path)?;
            let ui = UserInterface::Tui(tui);

            ui
        }
        Command::Modify(_) => todo!(),
        Command::Gui(_) => todo!(),
    };

    ui.run()?;
    Ok(())
}
