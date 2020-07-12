pub mod cli;
pub mod tui;

use std::path::PathBuf;

use self::cli::Cli;
use self::cli::Mode;
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

    let mut ui = match cli.mode {
        Mode::Tui => {
            let tui = Tui::new(cli.share_data_dir, cli.work_dir)?;
            let ui = UserInterface::Tui(tui);

            ui
        }
        Mode::Gui => todo!(),
    };

    ui.run()?;
    Ok(())
}
