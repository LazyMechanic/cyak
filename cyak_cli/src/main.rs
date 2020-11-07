pub mod app;
pub mod cli;

use cli::Cli;

fn main() {
    let app = || {
        init_logger();
        let cli = Cli::new()?;
        app::run(cli)
    };

    std::process::exit(match app() {
        Ok(_) => 0,
        Err(err) => {
            eprintln!("error: {}", err);
            1
        }
    });
}

fn init_logger() {
    let log_filters = std::env::var("RUST_LOG").unwrap_or("error".to_string());

    env_logger::Builder::new()
        .parse_filters(&log_filters)
        .format(|formatter, record| {
            use std::io::Write;

            writeln!(
                formatter,
                "{} [{}]: {}",
                chrono::Local::now().format("%Y-%m-%dT%H:%M:%S"),
                record.level(),
                record.args()
            )
        })
        .init();
}
