use std::path::PathBuf;

use super::ctx::Context;
use super::event::Event;
use super::terminal::Terminal;
use super::ui;
use super::Error;

use crossterm::event::KeyCode;

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
        let mut ctx = Context::default();
        loop {
            ctx.handle_event(self.term.next_event()?);

            if !ctx.is_running {
                break;
            }

            self.term.draw(|ref mut f| ui::draw(f, &ctx))?;
        }

        Ok(())
    }
}
