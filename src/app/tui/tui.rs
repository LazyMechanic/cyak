use std::path::PathBuf;

use super::terminal::Terminal;
use super::ui;
use super::Error;

pub struct Tui {
    term:      Terminal,
    work_path: PathBuf,
}

impl Tui {
    pub fn new(work_path: PathBuf) -> Result<Self, Error> {
        let term = Terminal::new()?;
        Ok(Self { term, work_path })
    }

    pub fn run(&mut self) -> Result<(), Error> {
        loop {
            self.term.draw(|ref mut f| ui::draw(f))?;
        }

        Ok(())
    }
}
