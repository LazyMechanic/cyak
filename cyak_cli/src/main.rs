pub mod app;
pub mod cli;

use cli::Cli;

fn main() {
    let app = || {
        let cli = Cli::new()?;
        app::run(cli)
    };

    std::process::exit(match app() {
        Ok(_) => 0,
        Err(err) => {
            eprintln!("error: {:?}", err);
            1
        }
    });
}
