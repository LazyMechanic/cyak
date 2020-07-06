use crossterm::event::{KeyEvent, MouseEvent};

/// Represents an event.
pub enum Event {
    /// A single key event with additional pressed modifiers.
    Key(KeyEvent),
    /// A single mouse event with additional pressed modifiers.
    Mouse(MouseEvent),
    /// An resize event with new dimensions after resize (columns, rows).
    Resize(u16, u16),
    /// Application tick
    Tick,
}

impl From<crossterm::event::Event> for Event {
    fn from(e: crossterm::event::Event) -> Self {
        match e {
            crossterm::event::Event::Key(v) => Event::Key(v),
            crossterm::event::Event::Mouse(v) => Event::Mouse(v),
            crossterm::event::Event::Resize(x, y) => Event::Resize(x, y),
        }
    }
}
