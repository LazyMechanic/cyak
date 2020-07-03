mod cli;

use cli::Cli;

fn main() {
    let cli = Cli::new();

    match cli {
        Cli::New(c) => cyak_lib::run_new(c.path, c.git),
        Cli::Modify(c) => cyak_lib::run_modify(c.path, c.git),
    }
}
