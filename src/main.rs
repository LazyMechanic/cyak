mod cli;

use anyhow::Result;

use cli::Cli;
use cli::Command;

fn main() -> Result<()> {
    let cli = Cli::new()?;

    match cli.command {
        Command::New(c) => cyak_lib::run_new(c.path),
        Command::Modify(c) => cyak_lib::run_modify(c.path),
    }

    Ok(())
}
