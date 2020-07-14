use std::path::PathBuf;

use super::ctx::Context;
use super::event::Event;
use super::terminal::Terminal;
use super::ui;
use super::Error;

use crossterm::event::{KeyCode, KeyModifiers};

pub struct Tui {
    term: Terminal,
    ctx:  Context,
}

impl Tui {
    pub fn new(share_data_dir: PathBuf, work_dir: PathBuf) -> Result<Self, Error> {
        if !share_data_dir.exists() {
            return Error::ShareDataDirNotFound.fail();
        }

        let term = Terminal::new()?;
        let ctx = Context::new(share_data_dir, work_dir);
        Ok(Self { term, ctx })
    }

    pub fn run(&mut self) -> Result<(), Error> {
        loop {
            match self.term.next_event()? {
                Event::Key(k) => {
                    let is_ctrl = k.modifiers.contains(KeyModifiers::CONTROL);
                    match k.code {
                        KeyCode::Char('c') => {
                            if is_ctrl {
                                self.ctx.is_running = false
                            }
                        }
                        _ => {}
                    }
                }
                _ => {}
            }

            if !self.ctx.is_running {
                break;
            }

            //let ctx = &self.ctx;
            //self.term.draw(|ref mut f| ui::draw(f, ctx))?;
        }

        Ok(())
    }
}
