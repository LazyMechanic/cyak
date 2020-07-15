pub mod ui;

use crate::cli::Cli;

pub fn run(cli: Cli) -> anyhow::Result<()> {
    let mut siv = cursive::default();

    siv.add_global_callback('q', |s| s.quit());

    siv.add_layer(cursive::views::TextView::new(
        "Hello cursive! Press <q> to quit.",
    ));

    siv.run();

    Ok(())
}
