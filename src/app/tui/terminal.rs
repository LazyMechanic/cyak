use std::io::{self, Stdout, Write};
use std::sync::mpsc;
use std::thread;
use std::time;

use crossterm::event::Event;
use crossterm::execute;
use crossterm::terminal::{
    disable_raw_mode,
    enable_raw_mode,
    EnterAlternateScreen,
    LeaveAlternateScreen,
};

use super::Error;
use std::ops::{Deref, DerefMut};

/// Terminal wrapper
pub struct Terminal {
    /// `tui-rs` terminal implementation
    term:         tui::Terminal<tui::backend::CrosstermBackend<Stdout>>,
    /// Thread for event handling
    event_handle: thread::JoinHandle<()>,
    /// Receiver for events
    event_rx:     mpsc::Receiver<Event>,
}

impl Terminal {
    pub fn new() -> Result<Self, Error> {
        enable_raw_mode()?;

        let mut stdout = io::stdout();
        execute!(stdout, EnterAlternateScreen)?;

        let backend = tui::backend::CrosstermBackend::new(stdout);
        let mut term = tui::Terminal::new(backend)?;

        term.hide_cursor()?;
        term.clear()?;

        let (tx, rx) = mpsc::channel();
        let event_handle = thread::spawn(move || {
            use crossterm::event as ce;
            loop {
                if ce::poll(time::Duration::from_secs(0)).unwrap() {
                    let e = ce::read().unwrap();
                    tx.send(e).unwrap();
                }
            }
        });

        Ok(Self {
            term,
            event_handle,
            event_rx: rx,
        })
    }

    pub fn next_event(&mut self) -> Result<Event, Error> {
        let e = self.event_rx.recv()?;
        Ok(e)
    }
}

impl Deref for Terminal {
    type Target = tui::Terminal<tui::backend::CrosstermBackend<Stdout>>;

    fn deref(&self) -> &Self::Target {
        &self.term
    }
}

impl DerefMut for Terminal {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.term
    }
}

impl Drop for Terminal {
    fn drop(&mut self) {
        disable_raw_mode().unwrap();
        execute!(self.term.backend_mut(), LeaveAlternateScreen).unwrap();
        self.term.show_cursor().unwrap();
    }
}
