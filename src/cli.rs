use std::path::PathBuf;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(name = "cyak", about = "Tool for create and modify cmake project")]
pub enum Cli {
    /// Create new project
    New(New),
    /// Try modify exists project
    Modify(Modify),
}

impl Cli {
    pub fn new() -> Self {
        Self::from_args()
    }
}

#[derive(Debug, StructOpt)]
pub struct New {
    #[structopt(short, long)]
    pub git:  bool,
    #[structopt(required = true, name = "path")]
    pub path: PathBuf,
}

#[derive(Debug, StructOpt)]
pub struct Modify {
    #[structopt(short, long)]
    pub git:  bool,
    #[structopt(required = true, name = "path")]
    pub path: PathBuf,
}
