use std::io::{self, Stdout, Write};
use std::ops::{Deref, DerefMut};
use std::sync::mpsc;
use std::sync::mpsc::SendError;
use std::thread;
use std::time;

use tui::backend::CrosstermBackend;

use crossterm::terminal::{
    disable_raw_mode,
    enable_raw_mode,
    EnterAlternateScreen,
    LeaveAlternateScreen,
};
use crossterm::{execute, ErrorKind};

use super::event::Event;
use super::Error;

const TICK_RATE: u64 = 250;

/// Terminal wrapper
pub struct Terminal {
    /// `tui-rs` terminal implementation
    term:         tui::Terminal<CrosstermBackend<Stdout>>,
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

        let backend = CrosstermBackend::new(stdout);
        let mut term = tui::Terminal::new(backend)?;

        term.hide_cursor()?;
        term.clear()?;

        let (tx, rx) = mpsc::channel();
        let event_handle = thread::spawn(move || {
            use crossterm::event as ce;

            let tick_rate = time::Duration::from_millis(TICK_RATE);
            let mut last_tick = time::Instant::now();
            loop {
                if ce::poll(tick_rate - last_tick.elapsed()).unwrap() {
                    let e: ce::Event = ce::read().unwrap();
                    let e: Event = e.into();
                    tx.send(e).unwrap();
                }

                if last_tick.elapsed() >= tick_rate {
                    tx.send(Event::Tick).unwrap();
                    last_tick = time::Instant::now();
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
    type Target = tui::Terminal<CrosstermBackend<Stdout>>;

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
