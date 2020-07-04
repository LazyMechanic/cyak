use super::terminal::Terminal;
use super::Error;

pub struct Tui {
    term: Terminal,
}

impl Tui {
    pub fn new() -> Result<Self, Error> {
        let term = Terminal::new()?;
        Ok(Self { term })
    }

    pub fn run(&mut self) -> Result<(), Error> {
        Ok(())
    }
}
