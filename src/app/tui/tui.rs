use std::path::PathBuf;

use super::ctx::Context;
use super::event::Event;
use super::terminal::Terminal;
use super::ui;
use super::Error;

use crossterm::event::KeyCode;

pub struct Tui {
    term: Terminal,
    ctx:  Context,
}

impl Tui {
    pub fn new(presets_dir: PathBuf, work_dir: PathBuf) -> Result<Self, Error> {
        if !presets_dir.exists() {
            return Error::PresetsDirNotFound.fail();
        }

        let term = Terminal::new()?;
        let ctx = Context::new(presets_dir, work_dir);
        Ok(Self { term, ctx })
    }

    pub fn run(&mut self) -> Result<(), Error> {
        loop {
            match self.term.next_event()? {
                Event::Key(k) => match k.code {
                    KeyCode::Backspace => {}
                    KeyCode::Enter => {}
                    KeyCode::Left => {}
                    KeyCode::Right => {}
                    KeyCode::Up => {}
                    KeyCode::Down => {}
                    KeyCode::Home => {}
                    KeyCode::End => {}
                    KeyCode::PageUp => {}
                    KeyCode::PageDown => {}
                    KeyCode::Tab => {}
                    KeyCode::BackTab => {}
                    KeyCode::Delete => {}
                    KeyCode::Insert => {}
                    KeyCode::F(_) => {}
                    KeyCode::Char('q') => self.ctx.is_running = false,
                    KeyCode::Char(_) => {}
                    KeyCode::Null => {}
                    KeyCode::Esc => {}
                },
                Event::Mouse(_) => {}
                Event::Resize(_, _) => {}
                Event::Tick => {}
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
